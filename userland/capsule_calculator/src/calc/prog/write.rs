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

use super::base::Base;

pub const RADIX_MAX: usize = 48;

const DIGITS: usize = 40;

fn push(out: &mut [u8], w: usize, byte: u8) -> usize {
    if w < out.len() {
        out[w] = byte;
        return w + 1;
    }
    w
}

fn to_digits(mut m: u128, radix: u32, tmp: &mut [u8; DIGITS]) -> usize {
    if m == 0 {
        tmp[0] = b'0';
        return 1;
    }
    let mut n = 0;
    while m > 0 && n < DIGITS {
        let d = (m % radix as u128) as u8;
        tmp[n] = if d < 10 { b'0' + d } else { b'A' + d - 10 };
        m /= radix as u128;
        n += 1;
    }
    n
}

pub fn write(value: i64, base: Base, out: &mut [u8]) -> usize {
    let neg = base.signed() && value < 0;
    let mag: u128 = if base.signed() {
        (value as i128).unsigned_abs()
    } else {
        value as u32 as u128
    };
    let mut tmp = [0u8; DIGITS];
    let mut n = to_digits(mag, base.radix(), &mut tmp);
    while n < base.pad() && n < DIGITS {
        tmp[n] = b'0';
        n += 1;
    }
    let group = base.group();
    let mut w = 0;
    if neg {
        w = push(out, w, b'-');
    }
    let mut i = n;
    while i > 0 {
        i -= 1;
        w = push(out, w, tmp[i]);
        if group > 0 && i > 0 && i % group == 0 {
            w = push(out, w, b' ');
        }
    }
    w
}
