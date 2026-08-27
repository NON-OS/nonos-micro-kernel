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

use super::metrics::{
    FOOT_BTN_H, FOOT_BTN_W, GAP, PAD_TIGHT, RAIL_W, RANK_ROWS, ROW_H, TABLE_HEAD_H,
};
use super::rect::{self, Rect};

pub const AWARD_ROWS: usize = 6;

fn stage(w: u32, h: u32) -> Rect {
    let c = rect::content(w, h);
    (c.0, c.1, c.2, c.3.saturating_sub(FOOT_BTN_H + GAP))
}

pub fn table(w: u32, h: u32) -> Rect {
    let s = stage(w, h);
    (s.0, s.1, s.2.saturating_sub(RAIL_W.min(s.2) + GAP), s.3)
}

pub fn awards(w: u32, h: u32) -> Rect {
    let s = stage(w, h);
    let rail_w = RAIL_W.min(s.2);
    (s.0 + s.2.saturating_sub(rail_w), s.1, rail_w, s.3)
}

pub fn head(w: u32, h: u32) -> Rect {
    let inner = rect::inset(table(w, h), PAD_TIGHT);
    (inner.0, inner.1, inner.2, TABLE_HEAD_H)
}

pub fn row(w: u32, h: u32, index: usize) -> Rect {
    let inner = rect::inset(table(w, h), PAD_TIGHT);
    let top = inner.1 + TABLE_HEAD_H;
    let band = (inner.0, top, inner.2, ROW_H);
    rect::row(band, index.min(RANK_ROWS - 1), ROW_H, 0)
}

pub fn row_at(w: u32, h: u32, x: i32, y: i32) -> Option<usize> {
    rect::index_at(RANK_ROWS, x, y, |i| row(w, h, i))
}

pub fn award_row(w: u32, h: u32, index: usize) -> Rect {
    let inner = rect::inset(awards(w, h), PAD_TIGHT);
    let top = inner.1 + TABLE_HEAD_H;
    rect::row((inner.0, top, inner.2, ROW_H), index, ROW_H, 0)
}

pub fn back(w: u32, h: u32) -> Rect {
    let c = rect::content(w, h);
    let y = c.1 + c.3.saturating_sub(FOOT_BTN_H);
    (c.0, y, FOOT_BTN_W, FOOT_BTN_H)
}

pub fn back_at(w: u32, h: u32, x: i32, y: i32) -> bool {
    rect::hit(back(w, h), x, y)
}
