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

//! Replacing a payload of the same length, without a window in which a crash
//! leaves the table lying.
//!
//! The appender writes a name once. That is right for capsule images and
//! wrong for the one kind of file a running system rewrites: a record it keeps
//! for itself, such as the wallet's sealed vault, whose second write used to be
//! refused.
//!
//! Overwriting in place cannot be made safe: whichever of payload and digest
//! is written first, a crash between them leaves the other stale, and the
//! next boot authenticates bytes the digest does not describe. So the new
//! payload goes to a free extent first, where nothing points at it, and then
//! the table entry is repointed and re-digested in one sector write. Before
//! that write the old record is whole; after it the new one is. The extent the
//! record left behind is free for the next replacement to reuse.
//!
//! A different length is still refused, because the table records length as
//! part of the entry this path does not rewrite, and the rule about what may
//! be replaced at all stays in `store_rules`.

use super::digest::digest16;
use super::error::BlkError;
use super::store_free::free_extent;
use super::store_entry::patch_entry;
use super::store_rules::permitted;
use super::store_toc::{TocEntry, MAX_TOTAL_BYTES};
use super::store_write::{commit_entry, write_payload};

/// Replace entry `index` of `entries` with `data`, which must be the same
/// length. `floor` is the first byte after the reserved table region and
/// `capacity_bytes` the device's size, both bounds on where the payload may go.
pub fn replace(
    toc: &[u8],
    index: usize,
    entries: &[TocEntry],
    floor: u64,
    capacity_bytes: u64,
    data: &[u8],
) -> Result<(), BlkError> {
    let entry = entries.get(index).ok_or(BlkError::BadContainer)?;
    permitted(&entry.name, entry.len, data.len())?;
    let live: u64 = entries.iter().map(|e| e.len).sum();
    if live.saturating_add(data.len() as u64) > MAX_TOTAL_BYTES {
        return Err(BlkError::BadLength);
    }
    let at = free_extent(entries, floor, data.len() as u64);
    if at.saturating_add(data.len() as u64) > capacity_bytes {
        return Err(BlkError::BadLength);
    }
    write_payload(at, data)?;
    let region = patch_entry(toc, index, at, &digest16(data))?;
    commit_entry(&region, index)
}
