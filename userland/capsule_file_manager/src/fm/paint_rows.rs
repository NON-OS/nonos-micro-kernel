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

use super::file_color::color;
use super::file_kind::kind_of;
use super::fmt_time::fmt_time;
use super::human_size::human_size;
use super::layout::{FIRST_ROW_Y, LIST_VISIBLE, ROW_HEIGHT};
use super::manifest::WIDTH;
use super::paint_clip::clip;
use super::paint_metrics::{DATE_END, NAME_MAX, SIZE_END, TEXT_LEFT};
use super::paint_right::paint_right;
use super::state::State;
use super::theme::{MUTED, SELECTED};

pub fn paint_rows(state: &State, fb: &mut PaintBuffer) {
    if state.entries.is_empty() {
        let msg: &[u8] = if state.filter.is_empty() { b"empty directory" } else { b"no matches" };
        fb.text(TEXT_LEFT, FIRST_ROW_Y, msg, MUTED);
        return;
    }
    let mut y = FIRST_ROW_Y;
    for (i, entry) in state.entries.iter().enumerate().skip(state.scroll).take(LIST_VISIBLE) {
        let selected = i == state.cursor;
        if selected {
            fb.fill_rect(TEXT_LEFT, y - 4, WIDTH - 2 * TEXT_LEFT, 18, 0x222F5AA0);
        }
        let name_color = if selected { SELECTED } else { color(kind_of(entry)) };
        fb.text(TEXT_LEFT, y, clip(entry.label.as_bytes(), NAME_MAX), name_color);
        if let Some(size) = entry.size {
            paint_right(fb, SIZE_END, y, human_size(size).as_bytes(), MUTED);
        }
        if entry.mtime != 0 {
            paint_right(fb, DATE_END, y, fmt_time(entry.mtime).as_bytes(), MUTED);
        }
        y += ROW_HEIGHT;
    }
}
