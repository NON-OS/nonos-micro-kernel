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

//! Growing a table and removing one. A column is one extra separator appended
//! to every row of the run, walked back to front so the byte offsets taken
//! before the first edit stay valid for the rest.

use alloc::vec::Vec;

use super::state::State;
use super::table_ops::blank_row;
use crate::doc::table::geom::col_count_of;
use crate::doc::table::syntax::PIPE;

impl State {
    pub fn insert_table_row(&mut self) -> bool {
        let Some(run) = self.table_run_at_caret() else { return false };
        let cols = col_count_of(&self.doc, run).max(1);
        let (block, _) = self.doc_pos(self.caret);
        let end = self.doc_byte(block, self.doc.blocks[block].text.len());
        let mut ins: Vec<u8> = Vec::new();
        ins.push(b'\n');
        ins.extend_from_slice(&blank_row(cols));
        if !self.apply_edit(end, 0, &ins) {
            return false;
        }
        self.caret = end + 2;
        true
    }

    pub fn insert_table_col(&mut self) -> bool {
        let Some((start, end)) = self.table_run_at_caret() else { return false };
        for i in (start..end).rev() {
            let Some(len) = self.doc.blocks.get(i).map(|b| b.text.len()) else { continue };
            let at = self.doc_byte(i, len);
            if !self.apply_edit(at, 0, &[PIPE]) {
                return false;
            }
        }
        true
    }

    pub fn delete_table(&mut self) -> bool {
        let Some((start, end)) = self.table_run_at_caret() else { return false };
        let last = end.saturating_sub(1);
        let mut from = self.doc_byte(start, 0);
        let mut to = self.doc_byte(last, self.doc.blocks[last].text.len());
        if to < self.len {
            to += 1;
        } else if from > 0 {
            from -= 1;
        }
        if !self.apply_edit(from, to.saturating_sub(from), &[]) {
            return false;
        }
        self.caret = from.min(self.len);
        true
    }
}
