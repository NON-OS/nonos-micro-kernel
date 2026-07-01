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

use crate::iface::dhcp::write_decimal_u8;

pub fn write_octet_quad(buf: &mut [u8; 64], mut pos: usize, quad: [u8; 4]) -> usize {
    for (i, &byte) in quad.iter().enumerate() {
        if i > 0 {
            buf[pos] = b'.';
            pos += 1;
        }
        pos = write_decimal_u8::write_decimal_u8(buf, pos, byte);
    }
    pos
}
