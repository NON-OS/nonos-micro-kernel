// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

pub(super) fn get(ch: u8) -> [u8; 16] {
    match ch {
        b'0' => [0, 0x3C, 0x66, 0x6E, 0x76, 0x66, 0x66, 0x66, 0x3C, 0, 0, 0, 0, 0, 0, 0],
        b'1' => [0, 0x18, 0x38, 0x18, 0x18, 0x18, 0x18, 0x18, 0x7E, 0, 0, 0, 0, 0, 0, 0],
        b'2' => [0, 0x3C, 0x66, 0x06, 0x0C, 0x18, 0x30, 0x60, 0x7E, 0, 0, 0, 0, 0, 0, 0],
        b'3' => [0, 0x3C, 0x66, 0x06, 0x1C, 0x06, 0x06, 0x66, 0x3C, 0, 0, 0, 0, 0, 0, 0],
        b'4' => [0, 0x0C, 0x1C, 0x3C, 0x6C, 0x7E, 0x0C, 0x0C, 0x0C, 0, 0, 0, 0, 0, 0, 0],
        b'5' => [0, 0x7E, 0x60, 0x7C, 0x06, 0x06, 0x06, 0x66, 0x3C, 0, 0, 0, 0, 0, 0, 0],
        b'6' => [0, 0x1C, 0x30, 0x60, 0x7C, 0x66, 0x66, 0x66, 0x3C, 0, 0, 0, 0, 0, 0, 0],
        b'7' => [0, 0x7E, 0x06, 0x0C, 0x18, 0x30, 0x30, 0x30, 0x30, 0, 0, 0, 0, 0, 0, 0],
        b'8' => [0, 0x3C, 0x66, 0x66, 0x3C, 0x66, 0x66, 0x66, 0x3C, 0, 0, 0, 0, 0, 0, 0],
        b'9' => [0, 0x3C, 0x66, 0x66, 0x3E, 0x06, 0x0C, 0x18, 0x38, 0, 0, 0, 0, 0, 0, 0],
        _ => [0, 0x7E, 0x42, 0x42, 0x42, 0x42, 0x42, 0x7E, 0, 0, 0, 0, 0, 0, 0, 0],
    }
}
