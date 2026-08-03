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
//! The first-byte fanout table.

extern crate alloc;

use alloc::vec::Vec;

use crate::oid::ObjectId;

/// Counts of ids whose first byte is at most each value.
///
/// This is what makes a lookup cheap: the first byte of an id picks a range
/// out of 256, and the search only ever covers that range instead of every
/// object in the pack.
pub(super) fn fanout(sorted: &[(ObjectId, u64)]) -> Vec<u32> {
    let mut counts = [0u32; 256];
    for (id, _) in sorted {
        counts[usize::from(id.as_bytes()[0])] += 1;
    }
    let mut out = Vec::with_capacity(256);
    let mut running = 0u32;
    for c in counts {
        running += c;
        out.push(running);
    }
    out
}
