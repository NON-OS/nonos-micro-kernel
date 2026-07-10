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

//! The left activity bar: a single Explorer button that shows the sidebar. It
//! is drawn as a small three-line list glyph with an accent rail when active.

use nonos_app_skeleton::PaintBuffer;

use super::layout::{ACTIVITY_W, TITLEBAR_H};
use super::theme;

pub(super) fn paint_activity(fb: &mut PaintBuffer, height: u32, sidebar_open: bool) {
    let th = theme::active();
    fb.fill_rect(0, 0, ACTIVITY_W, height, th.activity_bg);
    let icon_y = TITLEBAR_H + 18;
    if sidebar_open {
        fb.fill_rect(0, icon_y - 4, 2, 26, th.accent);
    }
    let color = if sidebar_open { th.icon_active } else { th.icon };
    let ix = 15;
    for k in 0..3 {
        fb.fill_rect(ix, icon_y + k * 7, 16, 2, color);
    }
}

// The activity bar occupies the leftmost strip; a click anywhere on it toggles
// the sidebar.
pub(super) fn activity_hit(x: i32) -> bool {
    x >= 0 && (x as u32) < ACTIVITY_W
}
