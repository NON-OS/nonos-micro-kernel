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

use super::metrics::{PANE_PAD, RAIL_W, READOUT_H};
use crate::calc::prog::BITS;

pub const COLS: i32 = 16;
pub const GAP: i32 = 4;
pub const BIT_H: i32 = 26;
pub const LABEL_H: i32 = 24;
pub const ROW_GAP: i32 = 4;
pub const ROW_STRIDE: i32 = BIT_H + LABEL_H + ROW_GAP;
pub const BAND_H: i32 = ROW_STRIDE * 2 - ROW_GAP;

pub fn top() -> i32 {
    PANE_PAD + READOUT_H + PANE_PAD
}

pub fn bottom() -> i32 {
    top() + BAND_H
}

pub fn band(win_w: i32) -> (i32, i32, i32, i32) {
    (RAIL_W + PANE_PAD, top(), win_w - RAIL_W - PANE_PAD * 2, BAND_H)
}

pub fn cell_w(win_w: i32) -> i32 {
    let (_, _, aw, _) = band(win_w);
    (aw - GAP * (COLS - 1)) / COLS
}

pub fn cell(win_w: i32, bit: u8) -> (i32, i32, i32, i32) {
    let (ax, ay, _, _) = band(win_w);
    let cw = cell_w(win_w);
    let idx = (BITS as i32 - 1) - bit as i32;
    let (row, col) = (idx / COLS, idx % COLS);
    (ax + col * (cw + GAP), ay + row * ROW_STRIDE, cw, BIT_H)
}

pub fn at(win_w: i32, x: i32, y: i32) -> Option<u8> {
    let (ax, ay, _, _) = band(win_w);
    let cw = cell_w(win_w);
    if cw <= 0 || x < ax || y < ay {
        return None;
    }
    let row = (y - ay) / ROW_STRIDE;
    let col = (x - ax) / (cw + GAP);
    if row >= BITS as i32 / COLS || (y - ay) % ROW_STRIDE >= BIT_H {
        return None;
    }
    if col >= COLS || (x - ax) % (cw + GAP) >= cw {
        return None;
    }
    Some(((BITS as i32 - 1) - (row * COLS + col)) as u8)
}
