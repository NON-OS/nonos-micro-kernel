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

use alloc::vec;

use crate::term::dimensions::{COLS, SCROLLBACK_ROWS, VISIBLE_ROWS};
use crate::term::grid::cell::Cell;
use crate::term::grid::types::Grid;
use crate::term::vt::color::{DEFAULT_BG, DEFAULT_FG};
use crate::term::vt::parser::Parser;
use crate::term::vt::utf8::Utf8;

impl Grid {
    pub fn new() -> Grid {
        Grid {
            cells: vec![Cell::blank(); COLS * VISIBLE_ROWS],
            alt: vec![Cell::blank(); COLS * VISIBLE_ROWS],
            history: vec![Cell::blank(); COLS * SCROLLBACK_ROWS],
            hist_head: 0,
            hist_count: 0,
            view_offset: 0,
            alternate: false,
            cursor_visible: true,
            x: 0,
            y: 0,
            fg: DEFAULT_FG,
            bg: DEFAULT_BG,
            flags: 0,
            parser: Parser::new(),
            utf8: Utf8::default(),
            total_scrolled: 0,
            scroll_top: 0,
            scroll_bot: VISIBLE_ROWS - 1,
        }
    }
}
