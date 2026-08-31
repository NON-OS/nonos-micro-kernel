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

//! One source of truth for ribbon geometry: the painter records the cells this
//! builds and the hit-test reads them back, so a click lands on the cell that
//! was drawn under it even though the pill labels change width with the caret.

use nonos_toolkit::ttf::line_height;

use super::items::RibbonItem;
use crate::editor::layout::{CHROME_PX, RIBBON_H, TABBAR_H, TITLEBAR_H};

pub(in crate::editor) struct RibbonCell {
    pub x0: u32,
    pub x1: u32,
    pub item: RibbonItem,
}

pub(super) struct Geom {
    pub cy: u32,
    pub ch: u32,
    pub ty: i32,
}

pub(super) const BAR_PAD: u32 = 12;
pub(super) const PILL_PAD: u32 = 10;
pub(super) const CHEVRON_W: u32 = 16;
pub(super) const GAP: u32 = 6;
pub(super) const SEP_GAP: u32 = 15;
pub(super) const SQUARE_W: u32 = 30;
pub(super) const DROP_PAD_X: u32 = 14;
pub(super) const DROP_SLACK: u32 = 24;

pub(super) fn band_top() -> u32 {
    TITLEBAR_H + TABBAR_H
}

pub(super) fn cell_h() -> u32 {
    line_height(CHROME_PX).max(14) as u32 + 6
}

pub(super) fn cell_top() -> u32 {
    band_top() + RIBBON_H.saturating_sub(cell_h()) / 2
}

pub(super) fn row_h() -> u32 {
    line_height(CHROME_PX).max(14) as u32 + 8
}

pub(super) fn text_top(box_h: u32) -> u32 {
    let lh = line_height(CHROME_PX).max(1) as u32;
    box_h.saturating_sub(lh) / 2
}
