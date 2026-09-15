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

//! Repointing one table entry: its offset and digest, in the one sector that
//! holds both.

use alloc::vec::Vec;

use super::error::BlkError;
use super::store_header::{ENTRY_LEN, HEADER_LEN};
use super::store_patch::{patch_digest, DIGEST_LEN, OFFSET_AT};

/// The table sector holding entry `index`'s offset and digest fields. They
/// begin at byte 128 * (index + 1), a boundary 512 divides into, and end 32
/// bytes later, so one sector always holds both.
pub fn entry_sector(index: usize) -> usize {
    (HEADER_LEN + ENTRY_LEN * index + OFFSET_AT) / 512
}

/// `toc` with entry `index` pointing at `offset` and carrying `digest`. Both
/// fields start at a 128-byte boundary and end within 32 bytes of it, so they
/// always share one sector; the caller commits that sector alone.
pub fn patch_entry(
    toc: &[u8],
    index: usize,
    offset: u64,
    digest: &[u8; DIGEST_LEN],
) -> Result<Vec<u8>, BlkError> {
    let mut region = patch_digest(toc, index, digest)?;
    let at = HEADER_LEN + ENTRY_LEN * index + OFFSET_AT;
    region[at..at + 8].copy_from_slice(&offset.to_le_bytes());
    Ok(region)
}
