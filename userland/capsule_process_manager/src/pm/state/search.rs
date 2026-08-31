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

use super::State;

// A process name is capped at PROC_NAME_LEN, so a query longer than this can
// never narrow anything further; a fixed buffer keeps the field allocation-free.
pub const QUERY_MAX: usize = 24;

// The head-band search field's whole model: what has been typed, and whether
// the field owns the keyboard. Focus lives here because the key handler has to
// suppress the letter shortcuts while the user is typing a process name.
pub struct Query {
    buf: [u8; QUERY_MAX],
    len: usize,
    focused: bool,
}

impl Query {
    pub fn new() -> Self {
        Query { buf: [0u8; QUERY_MAX], len: 0, focused: false }
    }
}

#[path = "search_match.rs"]
mod search_match;

impl State {
    // Every edit rewinds the scroll: the predicate below reshapes `filtered()`
    // under the viewport, and a scroll offset carried over from the wider list
    // would leave the table drawing rows that no longer exist.
    pub fn push_char(&mut self, byte: u8) {
        if (0x20..=0x7E).contains(&byte) && self.query.len < QUERY_MAX {
            self.query.buf[self.query.len] = byte;
            self.query.len += 1;
            self.scroll = 0;
        }
    }

    pub fn pop_char(&mut self) {
        self.query.len = self.query.len.saturating_sub(1);
        self.scroll = 0;
    }

    pub fn clear_query(&mut self) {
        self.query.len = 0;
        self.scroll = 0;
    }

    pub fn focus_search(&mut self, on: bool) {
        self.query.focused = on;
    }

    pub fn query(&self) -> &[u8] {
        &self.query.buf[..self.query.len]
    }

    pub fn query_focused(&self) -> bool {
        self.query.focused
    }
}
