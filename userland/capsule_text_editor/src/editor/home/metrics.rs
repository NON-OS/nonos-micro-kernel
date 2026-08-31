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

//! Home screen constants and left-rail geometry. Every rect the painters draw
//! and the router hit-tests is derived here or in `metrics_pane`, so a click
//! can never land somewhere the paint pass did not put the thing it hits.

use nonos_toolkit::ttf::line_height;

use crate::editor::layout::ACTIVITY_W;

pub(super) const RAIL_W: u32 = 246;
pub(super) const RAIL_PAD: u32 = 16;
pub(super) const PANE_PAD: u32 = 26;
pub(super) const CARD_W: u32 = 230;
pub(super) const CARD_PAD: u32 = 16;
pub(super) const COL_GAP: u32 = 26;
pub(super) const BRAND_SIDE: u32 = 28;
pub(super) const BRAND_Y: u32 = 18;
pub(super) const AVATAR: u32 = 32;
pub(super) const SEARCH_H: u32 = 44;
pub(super) const DOC_ICON: u32 = 34;
pub(super) const BODY: f32 = 17.0;
pub(super) const SUBHEAD: f32 = 19.0;
pub(super) const HEAD: f32 = 34.0;

pub(super) fn lh(px: f32) -> u32 {
    line_height(px).max(1) as u32
}

pub(super) fn rail_x() -> u32 {
    ACTIVITY_W
}

pub(super) fn pane_x() -> u32 {
    ACTIVITY_W + RAIL_W
}

pub(super) fn nav_rect() -> (u32, u32, u32) {
    let y = BRAND_Y + BRAND_SIDE + 22;
    (rail_x() + RAIL_PAD, y, RAIL_W - RAIL_PAD * 2)
}

pub(super) fn footer_h() -> u32 {
    lh(BODY) * 2 + 32
}
