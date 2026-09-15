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
// along with this program. If not, see <https://www.gnu.org/licenses/>.//! Where a replacement payload can go without touching a live extent.
//!
//! The container has no allocator and no free list; what it has is the table,
//! which names every live extent. A gap between two of them, or the space
//! after the last, is free by construction. The extent being replaced counts
//! as live here: its bytes must survive until the table points elsewhere, or
//! a crash in between would leave the old digest over new bytes.

use alloc::vec::Vec;

use super::store_toc::TocEntry;
use super::wire::SECTOR_SIZE;

/// The first sector-aligned position at or after `floor` where `len` bytes
/// fit without overlapping any extent in `entries`. When no gap fits, the
/// position after the last extent.
pub fn free_extent(entries: &[TocEntry], floor: u64, len: u64) -> u64 {
    let mut spans: Vec<(u64, u64)> =
        entries.iter().map(|e| (e.offset, sector_end(e.offset + e.len))).collect();
    spans.sort_unstable();
    let mut candidate = sector_end(floor);
    for (start, end) in spans {
        if candidate + len <= start {
            return candidate;
        }
        if end > candidate {
            candidate = end;
        }
    }
    candidate
}

fn sector_end(v: u64) -> u64 {
    let sector = SECTOR_SIZE as u64;
    v.div_ceil(sector) * sector
}
