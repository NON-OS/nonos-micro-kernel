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

//! The table's stored form: a run of `|`-delimited text lines. Cell text lives
//! in the document's own text buffer, so a table survives the full rebuild
//! `reflow` performs on every keystroke with no side structure to keep in step.

pub const PIPE: u8 = b'|';

pub fn is_row(text: &str) -> bool {
    let b = text.as_bytes();
    b.len() >= 2
        && b[0] == PIPE
        && b[b.len() - 1] == PIPE
        && b.iter().filter(|c| **c == PIPE).count() >= 2
}

pub fn col_count(text: &str) -> usize {
    match is_row(text) {
        true => text.bytes().filter(|c| *c == PIPE).count() - 1,
        false => 0,
    }
}

pub fn cell_span(text: &str, col: usize) -> Option<(usize, usize)> {
    if !is_row(text) {
        return None;
    }
    let mut start = 1usize;
    let mut seen = 0usize;
    for (i, c) in text.bytes().enumerate().skip(1) {
        if c != PIPE {
            continue;
        }
        if seen == col {
            return Some((start, i));
        }
        seen += 1;
        start = i + 1;
    }
    None
}

pub fn col_of_offset(text: &str, off: usize) -> usize {
    let n = col_count(text);
    if n == 0 {
        return 0;
    }
    let mut col = 0usize;
    for (i, c) in text.bytes().enumerate().take(off) {
        if c == PIPE && i > 0 {
            col += 1;
        }
    }
    col.min(n - 1)
}
