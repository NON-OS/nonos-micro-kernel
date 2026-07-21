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

use super::format_bits::format_bits;
use crate::matrix::Matrix;
use crate::version::Ecc;

/// Write both copies of the format information around the finders.
pub(crate) fn write_format(m: &mut Matrix, ecc: Ecc, mask: u8) {
    let bits = format_bits(ecc, mask);
    let n = m.n;
    for i in 0..15u16 {
        let b = (bits >> i) & 1 == 1;
        let (x1, y1) = match i {
            0..=5 => (8usize, i as usize),
            6 => (8, 7),
            7 => (8, 8),
            8 => (7, 8),
            _ => (14 - i as usize, 8),
        };
        m.set_format(x1, y1, b);
        let (x2, y2) = if i < 8 { (n - 1 - i as usize, 8) } else { (8, n - 15 + i as usize) };
        m.set_format(x2, y2, b);
    }
}
