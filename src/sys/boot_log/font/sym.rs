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

const D: [u8; 16] = [0, 0x7E, 0x42, 0x42, 0x42, 0x42, 0x42, 0x7E, 0, 0, 0, 0, 0, 0, 0, 0];

pub fn get_basic(c: u8) -> [u8; 16] {
    match c {
        b' ' => [0; 16],
        b'!' => [0, 0x18, 0x18, 0x18, 0x18, 0x18, 0, 0x18, 0, 0, 0, 0, 0, 0, 0, 0],
        b'"' => [0, 0x6C, 0x6C, 0x6C, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        b'#' => [0, 0x6C, 0x6C, 0xFE, 0x6C, 0xFE, 0x6C, 0x6C, 0, 0, 0, 0, 0, 0, 0, 0],
        b'$' => [0, 0x18, 0x3E, 0x60, 0x3C, 0x06, 0x7C, 0x18, 0, 0, 0, 0, 0, 0, 0, 0],
        b'%' => [0, 0x62, 0x66, 0x0C, 0x18, 0x30, 0x66, 0x46, 0, 0, 0, 0, 0, 0, 0, 0],
        b'&' => [0, 0x38, 0x6C, 0x38, 0x70, 0xDE, 0xCC, 0x76, 0, 0, 0, 0, 0, 0, 0, 0],
        b'\'' => [0, 0x18, 0x18, 0x30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        b'(' => [0, 0x0C, 0x18, 0x30, 0x30, 0x30, 0x18, 0x0C, 0, 0, 0, 0, 0, 0, 0, 0],
        b')' => [0, 0x30, 0x18, 0x0C, 0x0C, 0x0C, 0x18, 0x30, 0, 0, 0, 0, 0, 0, 0, 0],
        _ => D,
    }
}

pub fn get_math(c: u8) -> [u8; 16] {
    match c {
        b'*' => [0, 0, 0x66, 0x3C, 0xFF, 0x3C, 0x66, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        b'+' => [0, 0, 0x18, 0x18, 0x7E, 0x18, 0x18, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        b',' => [0, 0, 0, 0, 0, 0, 0x18, 0x18, 0x30, 0, 0, 0, 0, 0, 0, 0],
        b'-' => [0, 0, 0, 0, 0x7E, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        b'.' => [0, 0, 0, 0, 0, 0, 0x18, 0x18, 0, 0, 0, 0, 0, 0, 0, 0],
        b'/' => [0, 0x06, 0x0C, 0x18, 0x30, 0x60, 0xC0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        _ => D,
    }
}

pub fn get_punct(c: u8) -> [u8; 16] {
    match c {
        b':' => [0, 0, 0x18, 0x18, 0, 0x18, 0x18, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        b';' => [0, 0, 0x18, 0x18, 0, 0x18, 0x18, 0x30, 0, 0, 0, 0, 0, 0, 0, 0],
        b'<' => [0, 0x0C, 0x18, 0x30, 0x60, 0x30, 0x18, 0x0C, 0, 0, 0, 0, 0, 0, 0, 0],
        b'=' => [0, 0, 0, 0x7E, 0, 0x7E, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        b'>' => [0, 0x30, 0x18, 0x0C, 0x06, 0x0C, 0x18, 0x30, 0, 0, 0, 0, 0, 0, 0, 0],
        b'?' => [0, 0x3C, 0x66, 0x06, 0x0C, 0x18, 0, 0x18, 0, 0, 0, 0, 0, 0, 0, 0],
        b'@' => [0, 0x3C, 0x66, 0x6E, 0x6E, 0x60, 0x62, 0x3C, 0, 0, 0, 0, 0, 0, 0, 0],
        _ => D,
    }
}

pub fn get_bracket(c: u8) -> [u8; 16] {
    match c {
        b'[' => [0, 0x3C, 0x30, 0x30, 0x30, 0x30, 0x30, 0x3C, 0, 0, 0, 0, 0, 0, 0, 0],
        b'\\' => [0, 0xC0, 0x60, 0x30, 0x18, 0x0C, 0x06, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        b']' => [0, 0x3C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x3C, 0, 0, 0, 0, 0, 0, 0, 0],
        b'^' => [0, 0x10, 0x38, 0x6C, 0xC6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        b'_' => [0, 0, 0, 0, 0, 0, 0, 0, 0xFE, 0, 0, 0, 0, 0, 0, 0],
        b'`' => [0, 0x30, 0x18, 0x0C, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        b'{' => [0, 0x0E, 0x18, 0x18, 0x70, 0x18, 0x18, 0x0E, 0, 0, 0, 0, 0, 0, 0, 0],
        b'|' => [0, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0, 0, 0, 0, 0, 0, 0, 0],
        b'}' => [0, 0x70, 0x18, 0x18, 0x0E, 0x18, 0x18, 0x70, 0, 0, 0, 0, 0, 0, 0, 0],
        b'~' => [0, 0, 0, 0x76, 0xDC, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        _ => D,
    }
}
