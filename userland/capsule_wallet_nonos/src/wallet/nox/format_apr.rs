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

// Format an APR given in basis points as a percentage with two decimals, e.g.
// 1234 -> "12.34%". Returns the byte length written.
pub fn format_apr(bps: u64, out: &mut [u8]) -> usize {
    let whole = bps / 100;
    let frac = bps % 100;
    let mut tmp = [0u8; 20];
    let mut i = 0;
    let mut w = whole;
    loop {
        tmp[i] = b'0' + (w % 10) as u8;
        w /= 10;
        i += 1;
        if w == 0 {
            break;
        }
    }
    let need = i + 4; // "." + two decimals + "%"
    if need > out.len() {
        return 0;
    }
    for j in 0..i {
        out[j] = tmp[i - 1 - j];
    }
    out[i] = b'.';
    out[i + 1] = b'0' + (frac / 10) as u8;
    out[i + 2] = b'0' + (frac % 10) as u8;
    out[i + 3] = b'%';
    i + 4
}
