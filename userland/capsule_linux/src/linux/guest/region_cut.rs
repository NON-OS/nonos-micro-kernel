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

//! Taking a span out of the list of what a guest holds.

use alloc::vec::Vec;

use super::region::Region;

/// `regions` with `[at, at + len)` removed from every region it meets.
pub fn cut(regions: &[Region], at: u64, len: u64) -> Vec<Region> {
    let end = at.saturating_add(len);
    let mut out: Vec<Region> = Vec::with_capacity(regions.len());
    for r in regions {
        let r_end = r.at.saturating_add(r.len);
        if end <= r.at || at >= r_end {
            out.push(*r);
            continue;
        }
        if at > r.at {
            out.push(Region { at: r.at, len: at - r.at, ..*r });
        }
        if end < r_end {
            out.push(Region { at: end, len: r_end - end, ..*r });
        }
    }
    out
}
