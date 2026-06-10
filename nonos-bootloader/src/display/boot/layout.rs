// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! Splash geometry. Everything on the boot screen hangs off one
//! centered column derived from the live GOP mode, so the
//! composition holds at any resolution.

use crate::display::gop::get_dimensions;

pub const LINE_H: u32 = 16;
const PANEL_TITLE_H: u32 = 28;
const PANEL_PAD_BOTTOM: u32 = 12;
const MIN_ROWS: u32 = 10;
const MAX_ROWS: u32 = 22;

#[derive(Clone, Copy)]
pub struct Splash {
    pub col_x: u32,
    pub col_w: u32,
    pub wordmark_y: u32,
    pub subtitle_y: u32,
    pub chips_y: u32,
    pub panel_x: u32,
    pub panel_y: u32,
    pub panel_w: u32,
    pub panel_h: u32,
    pub bar_x: u32,
    pub bar_y: u32,
    pub bar_w: u32,
}

pub fn splash() -> Splash {
    let (w, h) = get_dimensions();
    let col_w = w.saturating_sub(120).min(840).max(320);
    let col_x = (w.saturating_sub(col_w)) / 2;
    let wordmark_y = h / 10;
    let subtitle_y = wordmark_y + 76;
    let chips_y = subtitle_y + 32;
    let panel_y = chips_y + 36;
    let bar_y = h.saturating_sub(h / 10);
    let panel_h = PANEL_TITLE_H + panel_rows_at(panel_y, bar_y) * LINE_H + PANEL_PAD_BOTTOM;
    Splash {
        col_x,
        col_w,
        wordmark_y,
        subtitle_y,
        chips_y,
        panel_x: col_x,
        panel_y,
        panel_w: col_w,
        panel_h,
        bar_x: col_x,
        bar_y,
        bar_w: col_w,
    }
}

pub fn panel_rows() -> u32 {
    let s = splash();
    panel_rows_at(s.panel_y, s.bar_y)
}

fn panel_rows_at(panel_y: u32, bar_y: u32) -> u32 {
    let room = bar_y
        .saturating_sub(48)
        .saturating_sub(panel_y)
        .saturating_sub(PANEL_TITLE_H + PANEL_PAD_BOTTOM);
    (room / LINE_H).clamp(MIN_ROWS, MAX_ROWS)
}

pub fn log_origin() -> (u32, u32) {
    let s = splash();
    (s.panel_x + 14, s.panel_y + PANEL_TITLE_H + 6)
}

pub fn log_line_width() -> u32 {
    splash().panel_w.saturating_sub(28)
}
