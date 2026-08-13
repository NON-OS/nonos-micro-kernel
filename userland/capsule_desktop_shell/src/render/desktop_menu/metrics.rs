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

//! Measured geometry and labels for the right-click menu.

use crate::render::measure_aa::measure_aa_bytes;
use crate::render::ui_font::{line_h, UI_PX};
use crate::state::Context;

/// Rows for the empty-desktop menu and the per-item menu. Row index doubles as
/// the action selector, matching the order the input handler switches on.
pub(super) const EMPTY_ITEMS: [&[u8]; 2] = [b"New Folder", b"New File"];
pub(super) const ITEM_ITEMS: [&[u8]; 3] = [b"Open", b"Rename", b"Delete"];

/// The rows the open menu should show, chosen by whether it targets an item.
pub(super) fn items(ctx: &Context) -> &'static [&'static [u8]] {
    if ctx.menu_target.is_some() {
        &ITEM_ITEMS
    } else {
        &EMPTY_ITEMS
    }
}

pub(super) const PAD_Y: u32 = 8;
pub(super) const PAD_X: u32 = 14;

/// Left edge of a row label: past the glyph column and its trailing gap.
pub(super) const LABEL_X: u32 = PAD_X + 32;

const ROW_LEAD: u32 = 12;
const MIN_W: u32 = 200;

/// Height of one row: the measured line box plus breathing room above and below.
pub(super) fn row_h() -> u32 {
    line_h(UI_PX) + ROW_LEAD
}

/// Panel width: wide enough for the longest label the open menu can show.
pub(super) fn width(ctx: &Context) -> u32 {
    let widest = items(ctx)
        .iter()
        .map(|label| measure_aa_bytes(label, UI_PX))
        .max()
        .unwrap_or(0);
    (LABEL_X + widest + PAD_X).max(MIN_W)
}
