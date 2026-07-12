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

//! The executable specification the differential proofs compare against, each
//! function restating the contract a Lean model formalizes, independent of the
//! implementation.

// Buddy order to size: verification/lean Nonos/Buddy.lean split_conserves. A
// block of order k spans two to the k bytes.
pub fn buddy_order_size(order: usize) -> usize {
    1usize << order
}

// Bitmap index arithmetic: verification/lean Nonos/Bitmap.lean. The byte and
// bit position of an index, restated with a modulo rather than the mask the
// implementation uses.
pub fn bitmap_byte_of(idx: usize) -> usize {
    idx / 8
}

pub fn bitmap_bit_mask(idx: usize) -> u8 {
    1u8 << (idx % 8)
}

// Region range algebra: verification/lean Nonos/Interval.lean and
// Nonos/Vma.lean. Two half-open ranges overlap unless one ends at or before the
// other begins; the two are exact negations.
pub fn region_overlaps(a0: u64, a1: u64, b0: u64, b1: u64) -> bool {
    a0 < b1 && b0 < a1
}

pub fn region_disjoint(a0: u64, a1: u64, b0: u64, b1: u64) -> bool {
    a1 <= b0 || b1 <= a0
}
