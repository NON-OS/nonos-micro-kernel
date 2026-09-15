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

//! The entry array of a directory record.
//!
//! An entry is at `base + i * 64`: a name in the first 56 bytes, a block
//! address in the next 8. Getting an index wrong here points one file's name
//! at another file's blocks, and the filesystem cannot tell afterwards.
//!
//! Free of the block device and of the cipher, so the whole of it is driven
//! on the host against a real block. The callers hold the key and the mount;
//! this holds the layout. The header lives in [`super::dir_block_header`],
//! which is also where the entry count is bounded.

use super::dir_block_header::count;
use super::dir_consts::{ENTRY_BYTES, NAME_BYTES, REC_ENTRY_BASE};
use super::dir_name_match::name_matches;
use super::read_u64::read_u64;

/// Every index below is bounded by `count`, which is itself clamped to
/// `MAX_ENTRIES`, so the offset this returns is always inside the block.
fn at(i: usize) -> usize {
    REC_ENTRY_BASE + i * ENTRY_BYTES
}

pub(super) fn name(block: &[u8], i: usize) -> &[u8] {
    let off = at(i);
    &block[off..off + NAME_BYTES]
}

pub(super) fn lba(block: &[u8], i: usize) -> u64 {
    read_u64(block, at(i) + NAME_BYTES)
}

/// The first entry whose name matches, searching only the entries the header
/// claims exist rather than the whole array.
pub(super) fn find(block: &[u8], wanted: &[u8]) -> Option<usize> {
    (0..count(block)).find(|&i| name_matches(name(block, i), wanted))
}
