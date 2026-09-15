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

//! The crash-safe replacement: where the new payload goes, what the table
//! entry says afterwards, and that both patched fields share one sector.

use alloc::string::String;
use alloc::vec::Vec;

use crate::vfs_blk::store_patch::store_header::{ENTRY_LEN, HEADER_LEN, MAX_ENTRIES};
use crate::vfs_blk::store_patch::store_toc::TocEntry;
use crate::vfs_blk::store_patch::{entry_sector, free_extent, DIGEST_AT, OFFSET_AT};

const SECTOR: u64 = 512;
const FLOOR: u64 = 8192;

fn extent(offset: u64, len: u64) -> TocEntry {
    TocEntry { name: String::from("/data/x"), offset, len, digest: [0; 16] }
}

#[test]
fn a_gap_between_live_extents_is_taken_before_the_end() {
    let live = [extent(FLOOR, 512), extent(FLOOR + 3 * SECTOR, 512)];
    assert_eq!(free_extent(&live, FLOOR, 400), FLOOR + SECTOR, "the two-sector hole");
    assert_eq!(free_extent(&live, FLOOR, 1500), FLOOR + 4 * SECTOR, "too big for it: the end");
}

#[test]
fn the_extent_being_replaced_is_never_chosen() {
    let live = [extent(FLOOR, 512)];
    let at = free_extent(&live, FLOOR, 512);
    assert!(at >= FLOOR + SECTOR, "the old bytes stay until the table points elsewhere");
}

#[test]
fn a_partial_sector_at_the_end_of_an_extent_is_not_reused() {
    let live = [extent(FLOOR, 100)];
    assert_eq!(free_extent(&live, FLOOR, 100), FLOOR + SECTOR);
}

#[test]
fn an_empty_table_places_at_the_floor_rounded_up_to_a_sector() {
    let none: Vec<TocEntry> = Vec::new();
    assert_eq!(free_extent(&none, FLOOR + 1, 10), FLOOR + SECTOR);
}

#[test]
fn offset_and_digest_of_every_entry_share_the_sector_that_is_committed() {
    for index in 0..MAX_ENTRIES {
        let base = HEADER_LEN + ENTRY_LEN * index;
        let sector = entry_sector(index);
        let (lo, hi) = (sector * 512, sector * 512 + 512);
        assert!(base + OFFSET_AT >= lo && base + OFFSET_AT + 8 <= hi, "offset of {index}");
        assert!(base + DIGEST_AT >= lo && base + DIGEST_AT + 16 <= hi, "digest of {index}");
    }
}
