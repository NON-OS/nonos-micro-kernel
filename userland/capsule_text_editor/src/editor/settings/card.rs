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

//! Geometry for the General card: the panel itself, its seven rows, and the
//! right-aligned control box each row hangs off its inner edge. Everything is
//! derived from the content width alone so the router can rebuild it without a
//! frame buffer in hand.

use super::geom::{head_top, lh, pane_x, HEAD_PX, PANE_PAD};

pub(super) const ROW_H: u32 = 56;
pub(super) const ROW_PAD: u32 = 20;
pub(super) const ROWS: usize = 7;
pub(super) const RADIUS: u32 = 14;
pub(super) const TOGGLE_W: u32 = 46;
pub(super) const TOGGLE_H: u32 = 25;
pub(super) const DROP_H: u32 = 34;
pub(super) const DROP_MIN_W: u32 = 190;

pub(super) const ROW_LABELS: [&str; ROWS] = [
    "Default font",
    "Font size",
    "Default view",
    "Show ruler",
    "Show status bar",
    "Open last document on startup",
    "Start with blank document",
];

pub(super) const DROP_VALUES: [&str; 3] = ["Inter", "12", "Page View"];

pub(super) fn card_rect(width: u32) -> (u32, u32, u32, u32) {
    let x = pane_x() + PANE_PAD;
    let y = head_top() + lh(HEAD_PX) + 18;
    let w = width.saturating_sub(x + PANE_PAD).max(1);
    (x, y, w, ROW_H * ROWS as u32)
}

pub(super) fn row_y(width: u32, row: usize) -> u32 {
    card_rect(width).1 + row as u32 * ROW_H
}

pub(super) fn control_box(width: u32, row: usize, w: u32, h: u32) -> (u32, u32, u32, u32) {
    let (cx, _, cw, _) = card_rect(width);
    let x = cx + cw.saturating_sub(ROW_PAD + w);
    (x, row_y(width, row) + ROW_H.saturating_sub(h) / 2, w, h)
}
