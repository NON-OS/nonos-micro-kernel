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

use super::div_wide::div_wide;
use super::mul_wide::mul_wide;

/// `a * b / d`, truncated, carrying the product at full 256-bit width.
///
/// `None` when `d` is zero, or when the quotient would not fit in 128 bits.
/// Both are refusals rather than saturations: a swap screen that cannot
/// compute the figure must say so, because a saturated number still looks
/// like a price.
pub fn mul_div(a: u128, b: u128, d: u128) -> Option<u128> {
    if d == 0 {
        return None;
    }
    let (hi, lo) = mul_wide(a, b);
    if hi >= d {
        return None;
    }
    Some(div_wide(hi, lo, d))
}
