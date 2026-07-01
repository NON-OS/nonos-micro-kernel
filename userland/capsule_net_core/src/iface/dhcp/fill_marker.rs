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

use crate::iface::dhcp::{write_decimal_u8, write_octet_quad};

pub fn fill_marker(buf: &mut [u8; 64], prefix_msg: &[u8], ip: [u8; 4], prefix: u8, gw: [u8; 4]) -> usize {
    let mut pos = 0usize;
    for &b in prefix_msg {
        buf[pos] = b;
        pos += 1;
    }
    pos = write_octet_quad::write_octet_quad(buf, pos, ip);
    buf[pos] = b'/';
    pos += 1;
    pos = write_decimal_u8::write_decimal_u8(buf, pos, prefix);
    buf[pos] = b' ';
    pos += 1;
    for &b in b"gw " {
        buf[pos] = b;
        pos += 1;
    }
    pos = write_octet_quad::write_octet_quad(buf, pos, gw);
    buf[pos] = b'\n';
    pos + 1
}
