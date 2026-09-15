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

//! Writing one entry's digest back into the table of contents.
//!
//! Split out from the replacement itself, and free of the block transport, so
//! it can be driven on the host against a real table. The table is the index
//! of everything the machine persists: an off-by-one here would point one
//! record's digest at another record's bytes, and every file after it would
//! fail its check on the next boot with nothing to say why.

use alloc::vec::Vec;

use super::error::BlkError;
use super::store_header::{ENTRY_LEN, HEADER_LEN};
use super::store_toc::NAME_LEN;

/// Offset of a record's payload offset field within the table.
pub const OFFSET_AT: usize = NAME_LEN;
/// Offset of a record's digest field within the table.
pub const DIGEST_AT: usize = NAME_LEN + 16;
pub const DIGEST_LEN: usize = 16;

/// `toc` with entry `index`'s digest replaced, or `BadContainer` when the
/// record does not lie entirely inside the region.
///
/// Bounds are checked rather than assumed. The region was read from a disk,
/// its entry count came from the same disk, and a container that claims more
/// entries than it carries must fail here rather than write past the table.
pub fn patch_digest(
    toc: &[u8],
    index: usize,
    digest: &[u8; DIGEST_LEN],
) -> Result<Vec<u8>, BlkError> {
    let base = HEADER_LEN
        .checked_add(ENTRY_LEN.checked_mul(index).ok_or(BlkError::BadContainer)?)
        .ok_or(BlkError::BadContainer)?;
    let at = base.checked_add(DIGEST_AT).ok_or(BlkError::BadContainer)?;
    let end = at.checked_add(DIGEST_LEN).ok_or(BlkError::BadContainer)?;
    if end > toc.len() || base + ENTRY_LEN > toc.len() {
        return Err(BlkError::BadContainer);
    }
    let mut region = toc.to_vec();
    region[at..end].copy_from_slice(digest);
    Ok(region)
}
