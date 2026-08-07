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

/// Divide the 256-bit value (`hi`, `lo`) by `d`, truncating toward zero.
///
/// Requires `hi < d`, which is what guarantees the quotient fits in 128
/// bits; `mul_div` checks that before calling. Restoring long division, one
/// bit at a time, because a 256-bit dividend has no primitive to lean on and
/// a quote must be the exact figure the chain would compute, not a rounded
/// one.
///
/// `rem` stays below `d` at every step, so the running value that the shift
/// builds is below `2 * d` and one subtraction is always enough. When the
/// shift pushes a set bit out of the top, the true value is at least 2^128
/// and so certainly above `d`, which is why that bit forces the subtraction
/// on its own. The wrapping subtraction is then exact: the true difference
/// is below 2^128.
pub fn div_wide(hi: u128, lo: u128, d: u128) -> u128 {
    let mut rem = hi;
    let mut quo = 0u128;
    let mut i = 128;
    while i > 0 {
        i -= 1;
        let carried_out = rem >> 127;
        rem = (rem << 1) | ((lo >> i) & 1);
        if carried_out == 1 || rem >= d {
            rem = rem.wrapping_sub(d);
            quo |= 1 << i;
        }
    }
    quo
}
