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

//! Colours and measurements for the menu bar. The panel and border colours are
//! the dock's, so the two bars share one look.

use crate::render::ui_font;

pub(super) const BAR_BG: u32 = 0xFF1B_2030;
pub(super) const BAR_BORDER: u32 = 0xFF2A_3446;
pub(super) const TILE_BG: u32 = 0xFF23_2C3C;
pub(super) const TILE_BORDER: u32 = 0xFF2E_3A4C;
pub(super) const FG: u32 = 0xFFCF_E6E9;
pub(super) const WORDMARK: u32 = 0xFFE6_F0FA;

const LOGO_X_LOGICAL: u32 = 12;
const LOGO_SIZE_LOGICAL: u32 = 20;
const WORDMARK_X_LOGICAL: u32 = 40;
const BRAND_RIGHT_LOGICAL: u32 = 118;

const RIGHT_MARGIN_LOGICAL: u32 = 12;
const PAD_X_LOGICAL: u32 = 12;
const GAP_LOGICAL: u32 = 10;

const BATT_GLYPH_W_LOGICAL: u32 = 24;
const NET_GLYPH_W_LOGICAL: u32 = 14;
const DOT_LOGICAL: u32 = 8;

pub(super) fn logo_x() -> u32 {
    LOGO_X_LOGICAL * ui_font::scale()
}

pub(super) fn logo_size() -> u32 {
    LOGO_SIZE_LOGICAL * ui_font::scale()
}

pub(super) fn wordmark_x() -> u32 {
    WORDMARK_X_LOGICAL * ui_font::scale()
}

/// Right edge of the clickable brand region (logo plus wordmark).
pub(super) fn brand_right() -> u32 {
    BRAND_RIGHT_LOGICAL * ui_font::scale()
}

pub(super) fn right_margin() -> u32 {
    RIGHT_MARGIN_LOGICAL * ui_font::scale()
}

pub(super) fn tile_h() -> u32 {
    crate::render::layout::menubar_tile_h()
}

pub(super) fn pad_x() -> u32 {
    PAD_X_LOGICAL * ui_font::scale()
}

pub(super) fn gap() -> u32 {
    GAP_LOGICAL * ui_font::scale()
}

pub(super) fn batt_glyph_w() -> u32 {
    BATT_GLYPH_W_LOGICAL * ui_font::scale()
}

pub(super) fn net_glyph_w() -> u32 {
    NET_GLYPH_W_LOGICAL * ui_font::scale()
}

pub(super) fn dot() -> u32 {
    DOT_LOGICAL * ui_font::scale()
}
