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

//! Shared Settings geometry. Every painter and every hit test reads its rects
//! from here, so a click can never land somewhere the row was not drawn.

use nonos_toolkit::ttf::line_height;

pub(super) const RAIL_X: u32 = 46;
pub(super) const RAIL_W: u32 = 250;
pub(super) const RAIL_PAD: u32 = 10;
pub(super) const PANE_PAD: u32 = 26;

pub(super) const TITLE_PX: f32 = 24.0;
pub(super) const HEAD_PX: f32 = 19.0;
pub(super) const NAV_PX: f32 = 17.0;
pub(super) const ROW_PX: f32 = 17.0;

pub(super) const NAV_LABELS: [&str; 7] = [
    "General",
    "Editing",
    "Auto Save",
    "Language",
    "Spelling & Grammar",
    "Collaboration",
    "Advanced",
];

pub(super) fn title_top() -> u32 {
    22
}

pub(super) fn head_top() -> u32 {
    26
}

pub(super) fn lh(px: f32) -> u32 {
    line_height(px).max(1) as u32
}

pub(super) fn nav_rect() -> (u32, u32, u32) {
    (RAIL_X + RAIL_PAD, title_top() + lh(TITLE_PX) + 18, RAIL_W - RAIL_PAD * 2)
}

pub(super) fn pane_x() -> u32 {
    RAIL_X + RAIL_W
}
