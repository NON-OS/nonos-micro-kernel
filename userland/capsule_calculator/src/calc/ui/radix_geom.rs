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

use super::bits_geom;
use super::metrics::{PANE_PAD, RAIL_W};

pub const ROW_H: i32 = 24;
pub const PAD: i32 = 8;
pub const GAP_ABOVE: i32 = 10;
pub const KEYPAD_GAP: i32 = 12;
pub const PANEL_H: i32 = ROW_H * 4 + PAD * 2;

pub fn top() -> i32 {
    bits_geom::bottom() + GAP_ABOVE
}

pub fn bottom() -> i32 {
    top() + PANEL_H
}

pub fn origin() -> (i32, i32) {
    (RAIL_W + PANE_PAD, top())
}

pub fn size(win_w: i32) -> (i32, i32) {
    (win_w - RAIL_W - PANE_PAD * 2, PANEL_H)
}

pub fn row_y(i: usize) -> i32 {
    top() + PAD + (i as i32) * ROW_H
}
