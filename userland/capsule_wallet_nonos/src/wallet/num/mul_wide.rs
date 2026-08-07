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

const HALF: u128 = u64::MAX as u128;

/// The exact 256-bit product of two 128-bit values, as (high, low).
///
/// A pool quote multiplies an amount by a reserve, and both are token
/// amounts in their smallest unit. Two figures of that size overflow 128
/// bits long before they overflow what a pool can hold, so the product has
/// to be carried at full width rather than saturated. Saturating here would
/// quote a price that is not the pool's.
///
/// Each half-product fits in 128 bits because the inputs are 64 bits wide,
/// so nothing in this function can itself overflow.
pub fn mul_wide(a: u128, b: u128) -> (u128, u128) {
    let (a1, a0) = (a >> 64, a & HALF);
    let (b1, b0) = (b >> 64, b & HALF);
    let p00 = a0 * b0;
    let p01 = a0 * b1;
    let p10 = a1 * b0;
    let p11 = a1 * b1;
    // Middle column: the carry out of the low half plus both cross terms.
    let mid = (p00 >> 64) + (p01 & HALF) + (p10 & HALF);
    let lo = (p00 & HALF) | (mid << 64);
    let hi = p11 + (p01 >> 64) + (p10 >> 64) + (mid >> 64);
    (hi, lo)
}
