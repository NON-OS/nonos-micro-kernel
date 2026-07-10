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

//! Whole-line editing: duplicate the caret line (Ctrl+D) and remove it
//! (Ctrl+Shift+K). Each is a single undoable edit and the caret keeps its
//! column on the line it ends up on.

use alloc::vec::Vec;

use super::line_bounds::{line_end, line_start};
use super::state::State;

impl State {
    // Copy the current line and insert the copy just below it, keeping the caret
    // at the same column on the new line.
    pub fn duplicate_line(&mut self) -> bool {
        let buf = &self.buf[..self.len];
        let ls = line_start(buf, self.caret);
        let le = line_end(buf, self.caret);
        let col = self.caret - ls;

        let mut ins = Vec::with_capacity(le - ls + 1);
        ins.push(b'\n');
        ins.extend_from_slice(&self.buf[ls..le]);
        if !self.apply_edit(le, 0, &ins) {
            return false;
        }
        self.clear_sel();
        self.caret = le + 1 + col;
        true
    }

    // Delete the current line including its trailing newline, dropping the caret
    // onto the line that takes its place.
    pub fn delete_line(&mut self) -> bool {
        let buf = &self.buf[..self.len];
        let ls = line_start(buf, self.caret);
        let le = line_end(buf, self.caret);
        // Take the newline after the line, or if it is the last line, the one
        // before it, so no blank remnant is left behind.
        let (from, to) = if le < self.len {
            (ls, le + 1)
        } else if ls > 0 {
            (ls - 1, le)
        } else {
            (ls, le)
        };
        if from == to {
            return self.apply_edit(ls, le - ls, b"");
        }
        if !self.apply_edit(from, to - from, b"") {
            return false;
        }
        self.clear_sel();
        self.caret = from.min(self.len);
        true
    }
}
