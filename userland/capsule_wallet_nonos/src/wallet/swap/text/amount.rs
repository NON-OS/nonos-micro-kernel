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

//! A token amount as a figure a reader can compare at a glance.

use super::digits::{pow10, u128_str};

/// Longest fraction worth showing. Past this the figure is harder to read
/// than it is precise, and the exact value is what the chain enforces.
const MAX_PLACES: usize = 4;

/// Write `v` as a decimal with `decimals` places, trailing zeros trimmed.
pub fn amount(v: u128, decimals: u8, out: &mut [u8]) -> usize {
    let scale = pow10(decimals);
    let mut n = u128_str(v / scale, out);
    let frac = v % scale;
    if frac == 0 || n + 2 >= out.len() {
        return n;
    }
    out[n] = b'.';
    n += 1;
    n = places(frac, scale, decimals, n, out);
    if n > 0 && out[n - 1] == b'.' {
        n -= 1;
    }
    n
}

/// Write the fraction, dropping the run of zeros it ends on.
fn places(frac: u128, scale: u128, decimals: u8, mut n: usize, out: &mut [u8]) -> usize {
    let keep = (decimals as usize).min(MAX_PLACES);
    let mut rem = frac;
    let mut shown = 0;
    let mut trailing = 0;
    while shown < keep && n < out.len() {
        let step = scale / pow10(shown as u8 + 1);
        let digit = (rem / step) as u8;
        rem %= step;
        out[n] = b'0' + digit;
        trailing = if digit == 0 { trailing + 1 } else { 0 };
        n += 1;
        shown += 1;
    }
    n - trailing
}
