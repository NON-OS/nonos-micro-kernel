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

use crate::calc::fixed::{Fixed, FRAC};

use super::constants::DISPLAY_MAX;
use super::fraction::{effective_fraction_digits, write_fraction};
use super::integer::write_u128;

pub fn format(value: Fixed, decimal_digits_typed: u8, out: &mut [u8]) -> usize {
    let neg = value < 0;
    let magnitude = if neg { (value as i128).unsigned_abs() } else { value as u128 };
    let int_part = magnitude / FRAC as u128;
    let frac_part = magnitude % FRAC as u128;
    let mut buf = [0u8; DISPLAY_MAX];
    let mut n = 0;
    if neg && n < buf.len() {
        buf[n] = b'-';
        n += 1;
    }
    n += write_u128(int_part, &mut buf[n..]);
    let frac_digits = effective_fraction_digits(frac_part, decimal_digits_typed);
    if frac_digits > 0 {
        if n < buf.len() {
            buf[n] = b'.';
            n += 1;
        }
        n += write_fraction(frac_part, frac_digits, &mut buf[n..]);
    }
    let len = n.min(out.len());
    out[..len].copy_from_slice(&buf[..len]);
    len
}
