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

//! Grid metrics for the desktop icons.

use crate::render::ui_font;

const ICON_LOGICAL: u32 = 30;
const CELL_W_LOGICAL: u32 = 102;
const CELL_H_LOGICAL: u32 = 96;
const LEFT_LOGICAL: u32 = 26;
const CARET_W_LOGICAL: u32 = 2;
const BOTTOM_RESERVE_LOGICAL: u32 = 120;

pub(super) fn icon() -> u32 {
    ICON_LOGICAL * ui_font::scale()
}

pub(super) fn cell_w() -> u32 {
    CELL_W_LOGICAL * ui_font::scale()
}

pub(super) fn cell_h() -> u32 {
    CELL_H_LOGICAL * ui_font::scale()
}

pub(super) fn left() -> u32 {
    LEFT_LOGICAL * ui_font::scale()
}

/// Width of the rename caret, which is a rule rather than a glyph cell.
pub(super) fn caret_w() -> u32 {
    CARET_W_LOGICAL * ui_font::scale()
}

/// Leave room at the bottom for the floating dock plus a little breathing space.
pub(super) fn bottom_reserve() -> u32 {
    BOTTOM_RESERVE_LOGICAL * ui_font::scale()
}
