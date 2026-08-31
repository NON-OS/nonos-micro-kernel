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

//! Creating a table. Every op goes through `apply_edit`, the one mutation path,
//! so a table is inserted, undone and redone by exactly the machinery that
//! already moves text, and `reflow` re-derives the grid from the result.

use alloc::vec::Vec;

use super::line_bounds::line_start;
use super::mode::Mode;
use super::state::State;
use crate::doc::table::run::run_of;
use crate::doc::table::syntax::PIPE;

pub const DEFAULT_ROWS: usize = 3;
pub const DEFAULT_COLS: usize = 3;

pub(in crate::editor) fn blank_row(cols: usize) -> Vec<u8> {
    let mut row = Vec::new();
    row.resize(cols + 1, PIPE);
    row
}

impl State {
    pub fn table_run_at_caret(&self) -> Option<(usize, usize)> {
        let (block, _) = self.doc_pos(self.caret);
        run_of(&self.doc, block)
    }

    pub fn insert_table(&mut self, rows: usize, cols: usize) -> bool {
        if self.mode != Mode::Document {
            return false;
        }
        let (rows, cols) = (rows.max(1), cols.max(1));
        let mut ins: Vec<u8> = Vec::new();
        for _ in 0..rows {
            ins.extend_from_slice(&blank_row(cols));
            ins.push(b'\n');
        }
        let at = line_start(&self.buf[..self.len], self.caret);
        if !self.apply_edit(at, 0, &ins) {
            return false;
        }
        self.caret = at + 1;
        true
    }
}
