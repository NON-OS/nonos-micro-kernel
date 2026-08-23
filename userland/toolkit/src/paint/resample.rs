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

pub fn source_span(index: u32, src: u32, dst: u32) -> (u64, u64) {
    let lo = ((index as u64 * src as u64) << 16) / dst as u64;
    let hi = (((index as u64 + 1) * src as u64) << 16) / dst as u64;
    (lo, core::cmp::max(hi, lo + 1))
}

pub fn cover(lo: u64, hi: u64, cell: u64) -> u64 {
    let c0 = cell << 16;
    let c1 = c0 + (1 << 16);
    let a = if lo > c0 { lo } else { c0 };
    let b = if hi < c1 { hi } else { c1 };
    if b > a {
        b - a
    } else {
        0
    }
}
