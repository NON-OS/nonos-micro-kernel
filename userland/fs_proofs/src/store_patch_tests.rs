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

//! Patching one record's digest, against a real table.
//!
//! The container holds every file the machine persists, so the property that
//! matters is not only that the right digest lands but that nothing else moves:
//! one wrong offset here and the next boot fails a digest on a record nobody
//! touched.

use alloc::vec;
use alloc::vec::Vec;

use crate::vfs_blk::store_patch::store_header::{ENTRY_LEN, HEADER_LEN};
use crate::vfs_blk::store_patch::store_toc::decode;
use crate::vfs_blk::store_patch::{patch_digest, DIGEST_AT, DIGEST_LEN};

const SECTOR: usize = 512;
const NAME_LEN: usize = 96;
const COUNT: usize = 3;
const CAPACITY: u64 = 1 << 20;

/// A table with three entries, laid out the way the appender writes one.
fn table() -> Vec<u8> {
    let mut toc = vec![0u8; region_len(COUNT)];
    toc[..8].copy_from_slice(b"NONOSTR1");
    toc[8..12].copy_from_slice(&1u32.to_le_bytes());
    toc[12..16].copy_from_slice(&(COUNT as u32).to_le_bytes());
    for i in 0..COUNT {
        let base = HEADER_LEN + ENTRY_LEN * i;
        let name = alloc::format!("/data/file{i}");
        toc[base..base + name.len()].copy_from_slice(name.as_bytes());
        let offset = (region_len(COUNT) + i * SECTOR) as u64;
        toc[base + NAME_LEN..base + NAME_LEN + 8].copy_from_slice(&offset.to_le_bytes());
        toc[base + NAME_LEN + 8..base + NAME_LEN + 16].copy_from_slice(&72u64.to_le_bytes());
        toc[base + DIGEST_AT..base + DIGEST_AT + DIGEST_LEN].copy_from_slice(&[i as u8 + 1; 16]);
    }
    toc
}

fn region_len(count: usize) -> usize {
    (HEADER_LEN + ENTRY_LEN * count).div_ceil(SECTOR) * SECTOR
}

#[test]
fn the_named_entry_gets_the_new_digest() {
    let toc = table();
    let out = patch_digest(&toc, 1, &[0xAB; DIGEST_LEN]).expect("in range");
    let entries = decode(&out, COUNT, CAPACITY).expect("still decodes");
    assert_eq!(entries[1].digest, [0xAB; DIGEST_LEN]);
}

#[test]
fn every_other_byte_is_untouched() {
    let toc = table();
    let out = patch_digest(&toc, 1, &[0xAB; DIGEST_LEN]).expect("in range");
    assert_eq!(out.len(), toc.len());
    let at = HEADER_LEN + ENTRY_LEN + DIGEST_AT;
    for i in 0..toc.len() {
        if (at..at + DIGEST_LEN).contains(&i) {
            continue;
        }
        assert_eq!(out[i], toc[i], "byte {i} moved");
    }
}

#[test]
fn no_neighbour_loses_its_extent_or_digest() {
    let toc = table();
    let before = decode(&toc, COUNT, CAPACITY).expect("decodes");
    let out = patch_digest(&toc, 1, &[0xAB; DIGEST_LEN]).expect("in range");
    let after = decode(&out, COUNT, CAPACITY).expect("decodes");
    for i in [0, 2] {
        assert_eq!(after[i].name, before[i].name);
        assert_eq!(after[i].offset, before[i].offset);
        assert_eq!(after[i].len, before[i].len);
        assert_eq!(after[i].digest, before[i].digest, "entry {i} digest changed");
    }
    assert_eq!(after[1].offset, before[1].offset, "the patched entry keeps its extent");
    assert_eq!(after[1].len, before[1].len);
}

#[test]
fn each_index_patches_its_own_record() {
    for i in 0..COUNT {
        let toc = table();
        let out = patch_digest(&toc, i, &[0x5A; DIGEST_LEN]).expect("in range");
        let entries = decode(&out, COUNT, CAPACITY).expect("decodes");
        for (j, e) in entries.iter().enumerate() {
            let want = if j == i { [0x5A; DIGEST_LEN] } else { [j as u8 + 1; DIGEST_LEN] };
            assert_eq!(e.digest, want, "patching {i} changed {j}");
        }
    }
}

#[test]
fn an_index_past_the_region_is_refused_rather_than_written() {
    let toc = table();
    /*
     * The entry count comes off the same disk as the region, so an index
     * beyond what the region holds is a container that claims more entries
     * than it carries. It has to fail rather than write past the table.
     */
    let room = (toc.len() - HEADER_LEN) / ENTRY_LEN;
    assert!(patch_digest(&toc, room, &[0; DIGEST_LEN]).is_err());
    assert!(patch_digest(&toc, room + 1, &[0; DIGEST_LEN]).is_err());
    assert!(patch_digest(&toc, usize::MAX, &[0; DIGEST_LEN]).is_err());
    assert!(patch_digest(&toc, usize::MAX / 2, &[0; DIGEST_LEN]).is_err());
}

#[test]
fn an_empty_or_short_region_is_refused() {
    assert!(patch_digest(&[], 0, &[0; DIGEST_LEN]).is_err());
    assert!(patch_digest(&[0u8; HEADER_LEN], 0, &[0; DIGEST_LEN]).is_err());
    let short = vec![0u8; HEADER_LEN + ENTRY_LEN - 1];
    assert!(patch_digest(&short, 0, &[0; DIGEST_LEN]).is_err());
}

#[test]
fn the_digest_field_sits_where_the_decoder_reads_it() {
    /*
     * The patch writes at NAME_LEN + 16 and the decoder reads at the same
     * place. They are separate files, so this pins them together rather than
     * trusting that two constants stay equal.
     */
    let mut toc = table();
    let at = HEADER_LEN + DIGEST_AT;
    toc[at..at + DIGEST_LEN].copy_from_slice(&[0x77; DIGEST_LEN]);
    let entries = decode(&toc, COUNT, CAPACITY).expect("decodes");
    assert_eq!(entries[0].digest, [0x77; DIGEST_LEN]);
}

/*
 * The two rules that decide whether an existing record may be overwritten.
 * They are the boundary between a caller and a staged capsule payload, since
 * the persist op is open to anything that can reach the server.
 */
mod rules {
    use crate::vfs_blk::store_patch::{in_capsule_tree, permitted, CAPSULE_TREE};

    const DATA: &str = "/data/wallet.vault";
    const BLOB: u64 = 72;

    #[test]
    fn a_same_length_data_record_may_be_rewritten() {
        assert!(permitted(DATA, BLOB, BLOB as usize).is_ok());
    }

    #[test]
    fn a_different_length_is_refused_in_either_direction() {
        assert!(permitted(DATA, BLOB, BLOB as usize + 1).is_err());
        assert!(permitted(DATA, BLOB, BLOB as usize - 1).is_err());
        assert!(permitted(DATA, BLOB, 0).is_err());
    }

    #[test]
    fn nothing_under_the_capsule_tree_may_be_rewritten() {
        for name in [
            "/capsules/wallet.elf",
            "/capsules/wallet.manifest.bin",
            "/capsules/a.nonos_id_cert.bin",
            "/capsules/",
        ] {
            assert!(permitted(name, BLOB, BLOB as usize).is_err(), "{name} was writable");
            assert!(in_capsule_tree(name), "{name} not recognised as the capsule tree");
        }
    }

    #[test]
    fn a_name_that_only_looks_like_the_capsule_tree_is_not_it() {
        for name in ["/capsules", "/capsulesx/a.elf", "/data/capsules/a.elf", "capsules/a.elf"] {
            assert!(!in_capsule_tree(name), "{name} was taken for the capsule tree");
        }
    }

    #[test]
    fn the_prefix_is_the_one_the_installer_gate_uses() {
        assert_eq!(CAPSULE_TREE, "/capsules/");
    }
}
