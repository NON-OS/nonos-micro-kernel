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

use crate::snake::grid::{BOARD_Y, COLS, MARGIN, ROWS};

pub struct Layout {
    pub cell: u32,
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

impl Layout {
    pub fn inset(&self) -> u32 {
        (self.cell / 16).max(1)
    }
}

pub fn compute(width: u32, height: u32) -> Layout {
    let avail_w = width.saturating_sub(2 * MARGIN).max(COLS as u32);
    let avail_h = height.saturating_sub(BOARD_Y + MARGIN).max(ROWS as u32);
    let cell = (avail_w / COLS as u32).min(avail_h / ROWS as u32).max(4);
    let w = cell * COLS as u32;
    let h = cell * ROWS as u32;
    let x = width.saturating_sub(w) / 2;
    let y = BOARD_Y + avail_h.saturating_sub(h) / 2;
    Layout { cell, x, y, w, h }
}
