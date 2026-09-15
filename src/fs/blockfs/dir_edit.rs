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

//! Adding and removing an entry within one record block.
//!
//! The two operations that move bytes. Both refuse rather than clamp: an
//! append to a full block and a removal of an index that is not there are
//! both bugs in the caller, and doing something reasonable with them would
//! hide the bug in a filesystem that cannot report it later.

use super::dir_block::name;
use super::dir_block_header::{count, has_room, set_count};
use super::dir_consts::{ENTRY_BYTES, NAME_BYTES, REC_ENTRY_BASE};
use super::write_u64::write_u64;

/// Append `(name, lba)`, returning its index. `None` when the block is full
/// or the name does not fit the field.
pub(super) fn append(block: &mut [u8], entry: &[u8], lba: u64) -> Option<usize> {
    if entry.is_empty() || entry.len() > NAME_BYTES || !has_room(block) {
        return None;
    }
    let i = count(block);
    let off = REC_ENTRY_BASE + i * ENTRY_BYTES;
    block[off..off + entry.len()].copy_from_slice(entry);
    for b in block[off + entry.len()..off + NAME_BYTES].iter_mut() {
        *b = 0;
    }
    write_u64(block, off + NAME_BYTES, lba);
    set_count(block, i + 1);
    Some(i)
}

/// Remove entry `i`, shifting the ones after it down so the used entries stay
/// a prefix of the block. Every reader stops at the count, so a gap would be
/// read as a live entry with a zeroed name.
pub(super) fn remove(block: &mut [u8], i: usize) -> bool {
    let n = count(block);
    if i >= n {
        return false;
    }
    for j in i..n - 1 {
        let dst = REC_ENTRY_BASE + j * ENTRY_BYTES;
        let src = REC_ENTRY_BASE + (j + 1) * ENTRY_BYTES;
        block.copy_within(src..src + ENTRY_BYTES, dst);
    }
    let last = REC_ENTRY_BASE + (n - 1) * ENTRY_BYTES;
    for b in block[last..last + ENTRY_BYTES].iter_mut() {
        *b = 0;
    }
    set_count(block, n - 1);
    true
}

/// Whether `i` names something. Used by proofs and by callers that want the
/// name back after an edit without recomputing the offset.
pub(super) fn entry_name(block: &[u8], i: usize) -> &[u8] {
    name(block, i)
}
