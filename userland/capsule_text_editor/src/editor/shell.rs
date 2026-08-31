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

//! Geometry shared by the shell's painter and its event router, so a click is
//! tested against the exact rectangle the corresponding panel was drawn in.

use super::layout::{ACTIVITY_W, RIBBON_H, SIDEBAR_W, TABBAR_H, TITLEBAR_H};

// Left edge of the code pane: past the activity bar, plus the sidebar when open.
pub(super) fn pane_x(sidebar_open: bool) -> u32 {
    ACTIVITY_W + if sidebar_open { SIDEBAR_W } else { 0 }
}

// Top edge of the code pane: below the menu bar, the tab strip, and the ribbon.
// The sidebar tree hangs off this too, and the ribbon band spans the whole
// window right of the activity bar, so both stay flush with no seam.
pub(super) fn pane_y() -> u32 {
    TITLEBAR_H + TABBAR_H + RIBBON_H
}

// The code pane rectangle for a given window size and sidebar state.
pub(super) fn pane_rect(width: u32, height: u32, sidebar_open: bool) -> (u32, u32, u32, u32) {
    let x = pane_x(sidebar_open);
    let y = pane_y();
    let w = width.saturating_sub(x);
    let h = height.saturating_sub(y + super::layout::FOOTER_H);
    (x, y, w, h)
}
