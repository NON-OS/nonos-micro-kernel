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

use crate::calc::fixed::MAX_FRACTION_DIGITS;

pub fn effective_fraction_digits(frac_part: u128, typed: u8) -> u32 {
    if typed > 0 {
        return typed as u32;
    }
    if frac_part == 0 {
        return 0;
    }
    let mut digits = MAX_FRACTION_DIGITS;
    let mut value = frac_part;
    while digits > 0 && value % 10 == 0 {
        value /= 10;
        digits -= 1;
    }
    digits
}

pub fn write_fraction(frac_part: u128, digits: u32, out: &mut [u8]) -> usize {
    let mut divisor: u128 = 1;
    for _ in 0..MAX_FRACTION_DIGITS {
        divisor *= 10;
    }
    let mut n = 0;
    let mut shown = 0u32;
    let mut remainder = frac_part;
    while shown < digits && n < out.len() {
        divisor /= 10;
        if divisor == 0 {
            break;
        }
        let digit = (remainder / divisor) as u8;
        out[n] = b'0' + digit;
        n += 1;
        remainder %= divisor;
        shown += 1;
    }
    n
}
