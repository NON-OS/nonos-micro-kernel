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

use alloc::string::String;
use alloc::vec::Vec;

const HEX: &[u8; 16] = b"0123456789abcdef";
// Bytes shown per hexdump row.
const COLS: usize = 16;
// Row ceiling so a large binary preview stays bounded.
const MAX_ROWS: usize = 4096;

// Render bytes as an offset/hex/ascii dump, one row per 16 bytes:
// `0000  48 65 6c 6c ...  Hello...`.
pub fn hex_lines(bytes: &[u8]) -> Vec<String> {
    let mut lines = Vec::new();
    for (row, chunk) in bytes.chunks(COLS).enumerate() {
        if row >= MAX_ROWS {
            break;
        }
        let mut line = String::with_capacity(8 + COLS * 4);
        push_offset(&mut line, row * COLS);
        line.push_str("  ");
        for i in 0..COLS {
            if let Some(&b) = chunk.get(i) {
                line.push(HEX[(b >> 4) as usize] as char);
                line.push(HEX[(b & 0xf) as usize] as char);
            } else {
                line.push_str("  ");
            }
            line.push(' ');
        }
        line.push(' ');
        for &b in chunk {
            line.push(if (0x20..=0x7e).contains(&b) { b as char } else { '.' });
        }
        lines.push(line);
    }
    lines
}

fn push_offset(line: &mut String, off: usize) {
    for shift in (0..16).step_by(4).rev() {
        line.push(HEX[(off >> shift) & 0xf] as char);
    }
}
