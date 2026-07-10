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

//! Block indent and dedent over the selected lines (Tab / Shift+Tab). The whole
//! affected region is replaced in a single undoable edit, and the selection is
//! kept over the same lines so repeated presses keep working.

use alloc::vec::Vec;

use super::line_bounds::{line_end, line_start};
use super::state::State;

const INDENT: &[u8] = b"    ";

impl State {
    pub fn indent_selection(&mut self) -> bool {
        self.reindent(true)
    }

    pub fn dedent_selection(&mut self) -> bool {
        self.reindent(false)
    }

    // True when a selection exists and spans more than one text line, i.e. the
    // case where Tab should indent lines rather than insert a tab stop.
    pub fn selection_is_multiline(&self) -> bool {
        match self.sel_range() {
            Some((s, e)) => self.buf[s..e].contains(&b'\n'),
            None => false,
        }
    }

    fn reindent(&mut self, add: bool) -> bool {
        // Whole lines: a partial selection still indents every line it touches.
        let (s, e) = self.sel_range().unwrap_or((self.caret, self.caret));
        let start = line_start(&self.buf[..self.len], s);
        let end = line_end(&self.buf[..self.len], e).max(start);
        let region: Vec<u8> = self.buf[start..end].to_vec();

        let mut out = Vec::with_capacity(region.len() + 16);
        let mut first = true;
        for line in region.split(|&b| b == b'\n') {
            if !first {
                out.push(b'\n');
            }
            first = false;
            if line.is_empty() {
                continue; // never indent a blank line
            }
            if add {
                out.extend_from_slice(INDENT);
                out.extend_from_slice(line);
            } else {
                out.extend_from_slice(&line[dedent_count(line)..]);
            }
        }

        let new_len = out.len();
        if !self.apply_edit(start, end - start, &out) {
            return false;
        }
        self.sel_anchor = Some(start);
        self.caret = start + new_len;
        true
    }
}

// How many leading bytes to drop when dedenting: one tab, or up to four spaces.
fn dedent_count(line: &[u8]) -> usize {
    if line.first() == Some(&b'\t') {
        return 1;
    }
    let mut n = 0;
    while n < INDENT.len() && line.get(n) == Some(&b' ') {
        n += 1;
    }
    n
}
