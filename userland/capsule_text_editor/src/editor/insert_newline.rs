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

//! Insert a newline that carries the current line's leading whitespace, so the
//! next line starts at the same indentation the way a code editor does.

use alloc::vec::Vec;

use super::state::State;

impl State {
    pub fn insert_newline(&mut self) -> bool {
        let mut line_start = self.caret;
        while line_start > 0 && self.buf[line_start - 1] != b'\n' {
            line_start -= 1;
        }
        let mut indent_end = line_start;
        while indent_end < self.caret && matches!(self.buf[indent_end], b' ' | b'\t') {
            indent_end += 1;
        }
        let mut text = Vec::with_capacity(1 + (indent_end - line_start));
        text.push(b'\n');
        text.extend_from_slice(&self.buf[line_start..indent_end]);
        self.insert(&text)
    }
}
