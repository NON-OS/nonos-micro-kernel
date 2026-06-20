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

use crate::term::grid::cell::Cell;
use crate::term::grid::types::Grid;
use crate::term::dimensions::{COLS, VISIBLE_ROWS, SCROLLBACK_ROWS};
use crate::term::vt::color::{DEFAULT_FG, DEFAULT_BG};

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
            x: 0,
            y: 0,
            fg: DEFAULT_FG,
            bg: DEFAULT_BG,
            flags: 0,
        }
    }
}
