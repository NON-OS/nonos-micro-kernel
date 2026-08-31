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

//! Bullet and numbered lists. A toggle rewrites the marker at the head of each
//! selected line through `apply_edit`, the one mutation path, so a list is
//! made, unmade, undone and redone by the machinery that already moves text.

use alloc::vec::Vec;

use super::line_bounds::{line_end, line_start};
use super::mode::Mode;
use super::state::State;
use crate::doc::list::ops::{marker, shifted};
use crate::doc::list::scan::{line_at, line_starts};
use crate::doc::list::syntax::{kind_of, marker_len, ListKind};

impl State {
    pub fn toggle_bullet_list(&mut self) -> bool {
        self.toggle_list(ListKind::Bullet)
    }

    pub fn toggle_numbered_list(&mut self) -> bool {
        self.toggle_list(ListKind::Number)
    }

    fn toggle_list(&mut self, kind: ListKind) -> bool {
        if self.mode != Mode::Document {
            return false;
        }
        let (from, to) = self.sel_range().unwrap_or((self.caret, self.caret));
        let start = line_start(&self.buf[..self.len], from);
        let end = line_end(&self.buf[..self.len], to);
        let starts = line_starts(&self.buf[..self.len], start, end);
        let strip =
            starts.iter().all(|s| kind_of(line_at(&self.buf[..self.len], *s)) == Some(kind));
        let mut caret = self.caret;
        for (i, s) in starts.iter().enumerate().rev() {
            let del = marker_len(line_at(&self.buf[..self.len], *s));
            let ins: Vec<u8> = match strip {
                true => Vec::new(),
                false => marker(kind, i + 1),
            };
            if del == 0 && ins.is_empty() {
                continue;
            }
            caret = shifted(caret, *s, del, ins.len());
            if !self.apply_edit(*s, del, &ins) {
                return false;
            }
        }
        self.sel_anchor = None;
        self.caret = caret.min(self.len);
        self.renumber_lists(start, starts.len());
        true
    }
}
