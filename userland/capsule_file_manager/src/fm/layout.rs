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

use super::manifest::WIDTH;
use super::state::{State, ViewKind};

// Curated list geometry. The row count is derived from the live window height
// each frame (see `measure`) so the list fills the window when resized or
// maximized instead of stranding a fixed 7 rows in the top-left corner.
pub const HEADER_H: u32 = 60;
pub const FOOTER_H: u32 = 38;
pub const ROW_H: u32 = 34;
pub const ICON_S: u32 = 20;
pub const PAD_X: u32 = 20;
// Left PLACES sidebar; the file list and chrome start at CONTENT_X.
pub const SIDEBAR_W: u32 = 216;
pub const CONTENT_X: u32 = SIDEBAR_W;
pub const SIDE_ROW_H: u32 = 34;
pub const SIDE_FIRST_Y: u32 = 92;

// Row/scroll geometry consumed by paint, click hit-testing, and scroll
// clamping. Kept as the first-row origin and default fallbacks.
pub const FIRST_ROW_Y: u32 = HEADER_H + 6;
pub const ROW_HEIGHT: u32 = ROW_H;
pub const LIST_VISIBLE: usize = 8;

// Icon-grid metrics: each cell holds one large icon and its label below it.
pub const GRID_TOP: u32 = HEADER_H + 12;
pub const GRID_CELL_W: u32 = 124;
pub const GRID_CELL_H: u32 = 112;
pub const GRID_ICON: u32 = 58;
pub const GRID_PAD_X: u32 = 24;

/// Recompute the geometry for the active view from the current window height
/// and stash it in state, so scroll clamping and click hit-testing use the same
/// numbers paint drew.
pub fn measure(state: &mut State, win_h: u32) {
    match state.view {
        ViewKind::List => {
            state.row_top = FIRST_ROW_Y;
            state.row_h = ROW_H;
            let avail = win_h.saturating_sub(FIRST_ROW_Y + FOOTER_H);
            state.view_rows = (avail / ROW_H).max(1) as usize;
        }
        ViewKind::Grid => {
            let content_w = WIDTH.saturating_sub(CONTENT_X + GRID_PAD_X);
            state.grid_cols = (content_w / GRID_CELL_W).max(1);
            let avail = win_h.saturating_sub(GRID_TOP + FOOTER_H);
            let rows = (avail / GRID_CELL_H).max(1);
            state.view_rows = (rows * state.grid_cols) as usize;
        }
    }
}
