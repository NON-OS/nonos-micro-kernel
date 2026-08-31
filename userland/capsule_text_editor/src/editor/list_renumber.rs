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

//! Keeping numbering sequential. A run is however many adjacent lines carry a
//! number marker, recovered from the text each time, so no toggle can leave a
//! stale index behind it or ahead of it.

use super::state::State;
use crate::doc::list::ops::{marker, shifted};
use crate::doc::list::scan::{line_at, next_line, run_start};
use crate::doc::list::syntax::{number_len, ListKind};

impl State {
    pub(super) fn renumber_lists(&mut self, start: usize, lines: usize) {
        let mut tail = start;
        for _ in 0..lines {
            match next_line(&self.buf[..self.len], tail) {
                Some(s) => tail = s,
                None => break,
            }
        }
        self.renumber_run(tail);
        self.renumber_run(start);
    }

    fn renumber_run(&mut self, at: usize) {
        let mut s = run_start(&self.buf[..self.len], at);
        let mut index = 1usize;
        let mut caret = self.caret;
        loop {
            let del = number_len(line_at(&self.buf[..self.len], s));
            if del == 0 {
                break;
            }
            let want = marker(ListKind::Number, index);
            if self.buf[s..s + del] != want[..] {
                caret = shifted(caret, s, del, want.len());
                if !self.apply_edit(s, del, &want) {
                    break;
                }
            }
            match next_line(&self.buf[..self.len], s) {
                Some(n) => s = n,
                None => break,
            }
            index += 1;
        }
        self.caret = caret.min(self.len);
    }
}
