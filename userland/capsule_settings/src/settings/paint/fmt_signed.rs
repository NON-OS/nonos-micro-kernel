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

use super::fmt_dec::fmt_dec;

pub fn fmt_signed(value: i32, out: &mut [u8; 5]) -> usize {
    let (sign, mag) = if value < 0 {
        out[0] = b'-';
        (1usize, (-value) as u32)
    } else {
        out[0] = b'+';
        (1usize, value as u32)
    };
    let mut dec = [0u8; 4];
    let n = fmt_dec(mag, &mut dec);
    out[sign..sign + n].copy_from_slice(&dec[..n]);
    sign + n
}
