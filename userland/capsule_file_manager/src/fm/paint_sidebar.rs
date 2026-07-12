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

use nonos_app_skeleton::PaintBuffer;

use super::icon;
use super::layout::{PAD_X, SIDEBAR_W, SIDE_FIRST_Y, SIDE_ROW_H};
use super::state::State;
use super::theme::{ACCENT, DIRECTORY, FOREGROUND, HEADER_BG, LINE, MUTED, SELECT_BG};

// Fixed quick-access locations. `(display, path)`; paths match how the vfs
// reports directory prefixes.
pub const PLACES: [(&str, &str); 3] =
    [("Root", "/"), ("Documents", "/docs/"), ("Capsules", "/capsules/")];

pub fn paint_sidebar(state: &State, fb: &mut PaintBuffer) {
    let h = fb.height;
    fb.fill_rect(0, 0, SIDEBAR_W, h, HEADER_BG);
    fb.fill_rect(SIDEBAR_W - 1, 0, 1, h, LINE);

    // section label, below the window titlebar
    fb.text(PAD_X, 40, b"PLACES", MUTED);

    for (i, (label, path)) in PLACES.iter().enumerate() {
        let y = SIDE_FIRST_Y + i as u32 * SIDE_ROW_H;
        let active = state.prefix.as_str() == *path;
        let row_bg = if active { SELECT_BG } else { HEADER_BG };
        if active {
            fb.fill_rect(8, y.saturating_sub(6), SIDEBAR_W - 16, SIDE_ROW_H - 2, SELECT_BG);
            fb.fill_rect(8, y.saturating_sub(6), 3, SIDE_ROW_H - 2, ACCENT);
        }
        icon::folder(fb, PAD_X, y, 18, DIRECTORY, row_bg);
        let c = if active { FOREGROUND } else { MUTED };
        let _ = fb.text_ttf((PAD_X + 30) as i32, (y.saturating_sub(2)) as i32, label, c, 18.0);
    }
}

/// If `y` lands on a place row, return its path for navigation.
pub fn place_at(y: u32) -> Option<&'static str> {
    if y < SIDE_FIRST_Y.saturating_sub(6) {
        return None;
    }
    let idx = ((y - SIDE_FIRST_Y.saturating_sub(6)) / SIDE_ROW_H) as usize;
    PLACES.get(idx).map(|(_, p)| *p)
}
