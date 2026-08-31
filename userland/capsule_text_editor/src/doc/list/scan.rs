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

//! Walking the flat text buffer by line. A marker is found and measured from
//! the bytes themselves each time, so nothing is held beside the buffer that a
//! reflow could leave pointing at the wrong line.

use alloc::vec::Vec;

use crate::doc::list::syntax::number_len;

pub fn line_at(buf: &[u8], start: usize) -> &[u8] {
    let start = start.min(buf.len());
    let end = buf[start..]
        .iter()
        .position(|b| *b == b'\n')
        .map(|p| start + p)
        .unwrap_or(buf.len());
    &buf[start..end]
}

pub fn line_starts(buf: &[u8], start: usize, end: usize) -> Vec<usize> {
    let mut out = Vec::new();
    out.push(start.min(buf.len()));
    for i in start..end.min(buf.len()) {
        if buf[i] == b'\n' {
            out.push(i + 1);
        }
    }
    out
}

pub fn next_line(buf: &[u8], start: usize) -> Option<usize> {
    let at = start.min(buf.len()) + line_at(buf, start).len();
    match at < buf.len() {
        true => Some(at + 1),
        false => None,
    }
}

pub fn run_start(buf: &[u8], at: usize) -> usize {
    let mut s = at.min(buf.len());
    while s > 0 {
        let prev = buf[..s - 1].iter().rposition(|b| *b == b'\n').map(|i| i + 1).unwrap_or(0);
        if number_len(line_at(buf, prev)) == 0 {
            break;
        }
        s = prev;
    }
    s
}
