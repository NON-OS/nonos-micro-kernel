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

use alloc::vec::Vec;

use crate::term::dimensions::COLS;
use crate::term::grid::cell::Cell;
use crate::term::vt::parser::Parser;
use crate::term::vt::utf8::Utf8;

pub struct Grid {
    pub cells: Vec<Cell>,
    pub alt: Vec<Cell>,
    pub history: Vec<Cell>,
    pub hist_head: usize,
    pub hist_count: usize,
    pub view_offset: usize,
    pub alternate: bool,
    pub cursor_visible: bool,
    pub x: usize,
    pub y: usize,
    pub fg: u32,
    pub bg: u32,
    pub flags: u8,
    pub parser: Parser,
    /// Rebuilds characters from the bytes they arrive in. It belongs to the
    /// grid because a character can be split across two feeds.
    pub utf8: Utf8,
    pub total_scrolled: u64,
    // DECSTBM vertical scroll region, inclusive 0-based rows. Defaults to the
    // whole screen; full-screen TUIs set a smaller window so a status line at
    // the top or bottom stays put while the body scrolls.
    pub scroll_top: usize,
    pub scroll_bot: usize,
}

impl Grid {
    pub fn idx(x: usize, y: usize) -> usize {
        y * COLS + x
    }
}
