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

//! Measured geometry for the titles and their drop-down panels. The painter
//! and the hit test both come through here, so a click lands on the glyphs it
//! was drawn under.

use super::items::{rows, title, TITLE_COUNT};
use crate::render::measure_aa::measure_aa;
use crate::render::topbar::brand_right;
use crate::render::ui_font::{line_h, scale, UI_PX};
use crate::state::Context;

const PAD_X_LOGICAL: u32 = 12;
const ROW_PAD_X_LOGICAL: u32 = 16;
const PAD_Y_LOGICAL: u32 = 8;
const ROW_LEAD_LOGICAL: u32 = 12;
const INSET_LOGICAL: u32 = 4;
const MIN_W_LOGICAL: u32 = 190;

pub(super) fn pad_x() -> u32 {
    PAD_X_LOGICAL * scale()
}

pub(super) fn row_pad_x() -> u32 {
    ROW_PAD_X_LOGICAL * scale()
}

pub(super) fn pad_y() -> u32 {
    PAD_Y_LOGICAL * scale()
}

pub(super) fn inset() -> u32 {
    INSET_LOGICAL * scale()
}

pub(super) fn row_h() -> u32 {
    line_h(UI_PX) + ROW_LEAD_LOGICAL * scale()
}

pub(super) fn title_w(ctx: &Context, index: usize) -> u32 {
    measure_aa(title(ctx, index), UI_PX) + pad_x() * 2
}

pub(super) fn title_x(ctx: &Context, index: usize) -> u32 {
    let mut x = brand_right();
    for i in 0..index.min(TITLE_COUNT) {
        x += title_w(ctx, i);
    }
    x
}

pub(super) fn panel_w(ctx: &Context, index: usize) -> u32 {
    let widest = rows(ctx, index).iter().map(|row| measure_aa(row, UI_PX)).max().unwrap_or(0);
    (widest + row_pad_x() * 2).max(MIN_W_LOGICAL * scale())
}

pub(super) fn panel_h(ctx: &Context, index: usize) -> u32 {
    rows(ctx, index).len() as u32 * row_h() + pad_y() * 2
}
