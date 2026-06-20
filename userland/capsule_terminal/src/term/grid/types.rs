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

use crate::term::grid::cell::Cell;
use crate::term::dimensions::COLS;
use crate::term::vt::parser::Parser;

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
    pub fg: u8,
    pub bg: u8,
    pub flags: u8,
    pub parser: Parser,
}

impl Grid {
    pub fn idx(x: usize, y: usize) -> usize {
        y * COLS + x
    }
}
