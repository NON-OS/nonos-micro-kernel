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
use super::icon;
use super::layout::{CONTENT_X, ICON_S, PAD_X};
use super::state::State;
use super::theme::{ACCENT, ALT_ROW, BACKGROUND, DIRECTORY, FILE_C, MUTED, SELECT_BG};

pub fn paint_rows(state: &State, fb: &mut PaintBuffer) {
    let w = fb.width;
    let cw = w.saturating_sub(CONTENT_X);
    let left = CONTENT_X + PAD_X;
    if state.entries.is_empty() {
        let msg: &str = if state.filter.is_empty() { "empty directory" } else { "no matches" };
        let _ = fb.text_ttf(left as i32, (state.row_top + 6) as i32, msg, MUTED, 18.0);
        return;
    }

    let name_x = left + ICON_S + 16;
    let date_end = w.saturating_sub(PAD_X);
    let size_end = date_end.saturating_sub(160);

    for (vis, (i, entry)) in
        state.entries.iter().enumerate().skip(state.scroll).take(state.view_rows).enumerate()
    {
        let y = state.row_top + vis as u32 * state.row_h;
        let selected = i == state.cursor;
        let row_bg = if selected {
            SELECT_BG
        } else if vis % 2 == 1 {
            ALT_ROW
        } else {
            BACKGROUND
        };
        if selected {
            fb.fill_rect(CONTENT_X, y, cw, state.row_h, SELECT_BG);
            fb.fill_rect(CONTENT_X, y, 3, state.row_h, ACCENT);
        } else if vis % 2 == 1 {
            fb.fill_rect(CONTENT_X, y, cw, state.row_h, ALT_ROW);
        }

        let is_dir = entry.label.as_bytes().last() == Some(&b'/');
        let icy = y + state.row_h.saturating_sub(ICON_S) / 2;
        if is_dir {
            icon::folder(fb, left, icy, ICON_S, DIRECTORY, row_bg);
        } else {
            icon::file(fb, left, icy, ICON_S, FILE_C, row_bg);
        }

        // name, centered in the row
        let name_color = color(kind_of(entry));
        let ny = y + state.row_h.saturating_sub(18) / 2;
        let _ = fb.text_ttf(name_x as i32, ny as i32, entry.label.as_str(), name_color, 18.0);

        // size + date, right-aligned, small mono, centered
        let my = y + state.row_h.saturating_sub(8) / 2;
        if let Some(size) = entry.size {
            right(fb, size_end, my, human_size(size).as_bytes(), MUTED);
        }
        if entry.mtime != 0 {
            right(fb, date_end, my, fmt_time(entry.mtime).as_bytes(), MUTED);
        }
    }
}

fn right(fb: &mut PaintBuffer, end_x: u32, y: u32, bytes: &[u8], c: u32) {
    let adv = fb.glyph_advance();
    let x = end_x.saturating_sub(bytes.len() as u32 * adv);
    fb.text(x, y, bytes, c);
}
