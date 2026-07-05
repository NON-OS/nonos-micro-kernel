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

// Longest line kept; the paint pass clips to the window, and a hard cap keeps a
// minified single-line file from turning into one enormous string.
const MAX_LINE: usize = 512;
// Ceiling on line count so a pathological file cannot exhaust the heap.
const MAX_LINES: usize = 8192;

// Split file bytes into printable lines: newlines break, carriage returns are
// dropped, tabs expand to spaces, and any other control byte becomes a dot so
// the glyph atlas never renders noise.
pub fn text_lines(bytes: &[u8]) -> Vec<String> {
    let mut lines = Vec::new();
    let mut line = String::new();
    for &b in bytes {
        if b == b'\n' {
            lines.push(core::mem::take(&mut line));
            if lines.len() >= MAX_LINES {
                return lines;
            }
            continue;
        }
        if b == b'\r' {
            continue;
        }
        if line.len() >= MAX_LINE {
            continue;
        }
        match b {
            b'\t' => line.push_str("    "),
            0x20..=0x7e => line.push(b as char),
            _ => line.push('.'),
        }
    }
    if !line.is_empty() || lines.is_empty() {
        lines.push(line);
    }
    lines
}
