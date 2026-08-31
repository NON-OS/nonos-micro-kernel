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

//! Metrics for the whole editor shell: an activity bar and file-explorer
//! sidebar on the left, a tab strip along the top, the code pane, and a status
//! bar at the bottom. The code pane's origin is computed from the chrome around
//! it, and the real monospace cell width is measured from the font each paint;
//! `GLYPH_ADVANCE` is only the pre-measure fallback.

// Window chrome and shell panels.
pub(super) const TITLEBAR_H: u32 = 26;
pub(super) const TABBAR_H: u32 = 32;
// The formatting ribbon between the tab strip and the pane. Chrome text clamps
// to 17px, so the row is tall enough for a 27px control cell plus its padding.
pub(super) const RIBBON_H: u32 = 40;
pub(super) const FOOTER_H: u32 = 26;
pub(super) const ACTIVITY_W: u32 = 46;
pub(super) const SIDEBAR_W: u32 = 220;

// Code pane internals, relative to the pane origin.
pub(super) const GUTTER_W: u32 = 54;
pub(super) const TEXT_PAD: u32 = 12;
pub(super) const PAD_X: u32 = 12;
pub(super) const PAD_TOP: u32 = 8;
pub(super) const ROW_H: u32 = 24;

// Font sizes in px. Chrome and status stay fixed; only the body zooms.
pub(super) const CHROME_PX: f32 = 14.0;
pub(super) const STATUS_PX: f32 = 13.0;

// Zoom bounds. Scale 2 is the default and reproduces the original 15px body.
pub(super) const MIN_SCALE: u32 = 1;
pub(super) const MAX_SCALE: u32 = 6;

// Body font size for a zoom level: 11 + 2*scale px, matching the terminal's
// zoom curve (scale 2 = 15px). Line height and gutter size track it so a zoom
// change moves everything together.
pub(super) fn body_px(scale: u32) -> f32 {
    11.0 + scale.clamp(MIN_SCALE, MAX_SCALE) as f32 * 2.0
}

pub(super) fn gutter_px(scale: u32) -> f32 {
    (body_px(scale) - 2.0).max(9.0)
}

pub(super) fn line_height(scale: u32) -> u32 {
    body_px(scale) as u32 + 7
}

/// Fallback cell width before the first measured advance is known.
pub(super) const GLYPH_ADVANCE: u32 = 9;

// The first text column's x, relative to the code pane's left edge.
pub(super) fn text_left(pane_x: u32) -> u32 {
    pane_x + GUTTER_W + TEXT_PAD
}

pub(super) fn wrap_cols(pane_w: u32, advance: u32) -> u32 {
    pane_w.saturating_sub(GUTTER_W + TEXT_PAD + PAD_X).saturating_div(advance.max(1)).clamp(16, 400)
}
