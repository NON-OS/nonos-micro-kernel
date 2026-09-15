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

//! The first bytes of a directory record: magic, entry count, next block.
//!
//! These are what a directory looks like before any entry is read, and they
//! all come off a disk this filesystem does not trust. The count in
//! particular decides how far the entry accessors will walk, so it is clamped
//! here rather than anywhere it is used.

use super::dir_consts::{
    ENTRY_BYTES, MAX_ENTRIES, REC_COUNT_OFFSET, REC_ENTRY_BASE, REC_MAGIC, REC_NEXT_OFFSET,
};
use super::read_u32::read_u32;
use super::read_u64::read_u64;
use super::write_u32::write_u32;
use super::write_u64::write_u64;

/// Every byte the entry accessors reach: the header and a full set of entries.
const RECORD_BYTES: usize = REC_ENTRY_BASE + MAX_ENTRIES * ENTRY_BYTES;

/// The length check covers the whole record, not just the header, so a short
/// block is refused here rather than indexed past its end by an accessor.
pub(super) fn is_record(block: &[u8]) -> bool {
    block.len() >= RECORD_BYTES && block[0..8] == REC_MAGIC[..]
}

/// Entries in this block, clamped to what it can hold. The stored count comes
/// off a disk, and a larger one would walk entries out of the block's own
/// bytes and into whatever follows the buffer.
pub(super) fn count(block: &[u8]) -> usize {
    (read_u32(block, REC_COUNT_OFFSET) as usize).min(MAX_ENTRIES)
}

pub(super) fn set_count(block: &mut [u8], n: usize) {
    write_u32(block, REC_COUNT_OFFSET, n as u32);
}

pub(super) fn has_room(block: &[u8]) -> bool {
    count(block) < MAX_ENTRIES
}

/// The next block of the chain, or zero at the end of it. Chaining is what
/// replaced the old seven-entry hard cap on a directory.
pub(super) fn next(block: &[u8]) -> u64 {
    read_u64(block, REC_NEXT_OFFSET)
}

pub(super) fn set_next(block: &mut [u8], lba: u64) {
    write_u64(block, REC_NEXT_OFFSET, lba);
}
