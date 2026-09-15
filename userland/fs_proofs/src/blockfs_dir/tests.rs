// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! What the directory layout has to guarantee, against a real block.
//!
//! The block here is a plain 512-byte array, which is what the block device
//! hands the caller. Every value in it is treated as attacker-controlled:
//! these records come off a disk the system does not trust, and the count
//! field in particular decides how far the entry accessors walk.

use super::dir_block::{find, lba, name};
use super::dir_block_header::{count, has_room, is_record, next, set_count, set_next};
use super::dir_consts::{ENTRY_BYTES, MAX_ENTRIES, NAME_BYTES, REC_ENTRY_BASE, REC_MAGIC};
use super::write_u64::write_u64;

const BLOCK: usize = 512;

fn empty() -> [u8; BLOCK] {
    let mut b = [0u8; BLOCK];
    b[0..8].copy_from_slice(&REC_MAGIC);
    b
}

/// Put `n` at entry `i` the way the writer does, so the readers are tested
/// against the layout rather than against a restatement of themselves.
fn put(b: &mut [u8], i: usize, n: &[u8], addr: u64) {
    let off = REC_ENTRY_BASE + i * ENTRY_BYTES;
    b[off..off + NAME_BYTES].fill(0);
    b[off..off + n.len()].copy_from_slice(n);
    write_u64(b, off + NAME_BYTES, addr);
}

#[test]
fn the_entry_array_fits_inside_one_block() {
    /*
     * The arithmetic every accessor depends on. If this ever stops holding,
     * `name` indexes past the block and the bound in `count` stops helping.
     */
    assert_eq!(REC_ENTRY_BASE + MAX_ENTRIES * ENTRY_BYTES, 472);
    /*
     * These are constants, so they are checked when this crate is compiled
     * rather than when the test runs: a change to the layout that no longer
     * fits a block fails the build instead of waiting for `cargo test`.
     */
    const { assert!(REC_ENTRY_BASE + MAX_ENTRIES * ENTRY_BYTES <= BLOCK) };
    const { assert!(NAME_BYTES < ENTRY_BYTES, "a name must leave room for an address") };
}

#[test]
fn an_entry_reads_back_what_was_written() {
    let mut b = empty();
    for i in 0..MAX_ENTRIES {
        put(&mut b, i, b"file", 1000 + i as u64);
    }
    set_count(&mut b, MAX_ENTRIES);
    for i in 0..MAX_ENTRIES {
        assert_eq!(lba(&b, i), 1000 + i as u64, "entry {i}");
    }
}

#[test]
fn entries_do_not_overlap() {
    // One file's name must never be readable through another file's slot.
    let mut b = empty();
    for i in 0..MAX_ENTRIES {
        let n = [b'a' + i as u8; 8];
        put(&mut b, i, &n, i as u64);
    }
    set_count(&mut b, MAX_ENTRIES);
    for i in 0..MAX_ENTRIES {
        assert_eq!(name(&b, i)[0], b'a' + i as u8, "slot {i} reads another slot");
        assert_eq!(lba(&b, i), i as u64);
    }
}

#[test]
fn a_lying_count_cannot_walk_past_the_entries() {
    /*
     * The count is four bytes off a disk. Every value it can hold, including
     * the ones that would index past the block, has to clamp.
     */
    let mut b = empty();
    for raw in [MAX_ENTRIES as u32 + 1, 8, 100, u32::MAX] {
        b[8..12].copy_from_slice(&raw.to_le_bytes());
        assert!(count(&b) <= MAX_ENTRIES, "count {raw} was not clamped");
        // The clamp is what makes this loop safe; it panics if it is not.
        for i in 0..count(&b) {
            let _ = name(&b, i);
            let _ = lba(&b, i);
        }
    }
}

#[test]
fn a_full_block_reports_no_room() {
    let mut b = empty();
    for n in 0..MAX_ENTRIES {
        set_count(&mut b, n);
        assert!(has_room(&b), "{n} of {MAX_ENTRIES} should have room");
    }
    set_count(&mut b, MAX_ENTRIES);
    assert!(!has_room(&b));
    // Including when the disk claims more than the block can hold.
    b[8..12].copy_from_slice(&u32::MAX.to_le_bytes());
    assert!(!has_room(&b));
}

#[test]
fn a_name_matches_only_in_full() {
    let mut b = empty();
    put(&mut b, 0, b"report", 7);
    put(&mut b, 1, b"report.txt", 8);
    set_count(&mut b, 2);

    assert_eq!(find(&b, b"report"), Some(0));
    assert_eq!(find(&b, b"report.txt"), Some(1));
    // A prefix is not a match: the rest of the field has to be zero.
    assert_eq!(find(&b, b"rep"), None);
    assert_eq!(find(&b, b"report.tx"), None);
    assert_eq!(find(&b, b"missing"), None);
}

#[test]
fn find_ignores_entries_past_the_count() {
    // Stale bytes from a removed entry must not resolve as a file.
    let mut b = empty();
    put(&mut b, 0, b"live", 1);
    put(&mut b, 1, b"deleted", 2);
    set_count(&mut b, 1);
    assert_eq!(find(&b, b"live"), Some(0));
    assert_eq!(find(&b, b"deleted"), None);
}

#[test]
fn an_overlong_name_is_refused_rather_than_truncated() {
    let mut b = empty();
    put(&mut b, 0, &[b'x'; NAME_BYTES], 1);
    set_count(&mut b, 1);
    let too_long = [b'x'; NAME_BYTES + 1];
    assert_eq!(find(&b, &too_long), None, "a longer name must not match a full field");
    assert_eq!(find(&b, &[b'x'; NAME_BYTES]), Some(0));
}

#[test]
fn a_block_that_is_not_a_record_is_refused() {
    assert!(is_record(&empty()));
    let mut wrong = empty();
    wrong[7] = b'2';
    assert!(!is_record(&wrong), "the magic must be checked in full");
    /*
     * Short blocks are refused before any offset is used, and a header
     * alone is short: the entry accessors reach seven entries past it.
     */
    assert!(!is_record(&[]));
    assert!(!is_record(&empty()[..REC_ENTRY_BASE]));
    assert!(!is_record(&empty()[..REC_ENTRY_BASE + MAX_ENTRIES * ENTRY_BYTES - 1]));
    assert!(is_record(&empty()[..REC_ENTRY_BASE + MAX_ENTRIES * ENTRY_BYTES]));
}

#[test]
fn the_chain_pointer_does_not_disturb_the_header() {
    let mut b = empty();
    set_count(&mut b, 5);
    set_next(&mut b, 0xDEAD_BEEF_1234_5678);
    assert_eq!(next(&b), 0xDEAD_BEEF_1234_5678);
    assert_eq!(count(&b), 5, "setting the chain pointer moved the count");
    assert!(is_record(&b), "setting the chain pointer moved the magic");
    // And nothing reached the first entry.
    assert_eq!(&b[REC_ENTRY_BASE..REC_ENTRY_BASE + 8], &[0u8; 8]);
}

#[test]
fn a_volume_written_before_chaining_reads_as_one_record() {
    /*
     * The compatibility claim behind putting the pointer in what were unused
     * header bytes: an old record has zeros there, and zero is the end of the
     * chain, so it reads as a single block rather than as a chain into LBA 0.
     */
    let mut old = empty();
    put(&mut old, 0, b"notes", 42);
    set_count(&mut old, 1);
    assert_eq!(next(&old), 0, "an old record must terminate the chain");
    assert_eq!(find(&old, b"notes"), Some(0));
    assert_eq!(lba(&old, 0), 42);
}
