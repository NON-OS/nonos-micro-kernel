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

use alloc::vec::Vec;

// The 12-byte sfnt header for `num` tables: flavor then the binary-search
// fields the format derives from the table count.
pub(super) fn sfnt_header(flavor: &[u8], num: usize) -> Vec<u8> {
    let mut range = 1usize;
    let mut entry_selector = 0u16;
    while range * 2 <= num {
        range *= 2;
        entry_selector += 1;
    }
    let search_range = (range * 16) as u16;
    let mut out = Vec::with_capacity(12);
    out.extend_from_slice(flavor);
    out.extend_from_slice(&(num as u16).to_be_bytes());
    out.extend_from_slice(&search_range.to_be_bytes());
    out.extend_from_slice(&entry_selector.to_be_bytes());
    out.extend_from_slice(&((num as u16) * 16 - search_range).to_be_bytes());
    out
}
