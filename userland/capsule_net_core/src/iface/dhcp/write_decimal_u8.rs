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

pub fn write_decimal_u8(buf: &mut [u8; 64], mut pos: usize, val: u8) -> usize {
    if val >= 100 {
        buf[pos] = b'0' + val / 100;
        pos += 1;
    }
    if val >= 10 {
        buf[pos] = b'0' + (val / 10) % 10;
        pos += 1;
    }
    buf[pos] = b'0' + val % 10;
    pos + 1
}
