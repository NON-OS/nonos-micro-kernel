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

use alloc::string::String;

use nonos_app_skeleton::paint::PaintBuffer;

use super::layout::{EDGE, TOP};
use super::rows::{list_top, row_rect, visible_rows};
use super::text::{BODY_PX, TITLE_PX};
use super::theme;
use crate::catalog::entry::file_name;

const TEXT_PAD: u32 = 12;

pub fn paint_library(fb: &mut PaintBuffer, items: &[String], sel: usize, scroll: usize) {
    fb.fill_rect(0, 0, fb.width, fb.height, theme::BAR_BG);
    fb.text_ttf(EDGE as i32, TOP as i32, "Videos", theme::TEXT, TITLE_PX);
    if items.is_empty() {
        let msg = "No .avi files in storage";
        fb.text_ttf(EDGE as i32, list_top() as i32, msg, theme::TEXT_DIM, BODY_PX);
        return;
    }
    for slot in 0..visible_rows(fb.height) {
        let Some(path) = items.get(scroll.saturating_add(slot)) else {
            break;
        };
        let r = row_rect(fb.width, slot);
        let selected = scroll.saturating_add(slot) == sel;
        if selected {
            fb.fill_rect(r.x, r.y, r.w, r.h, theme::TRACK);
        }
        let color = if selected { theme::TEXT } else { theme::TEXT_DIM };
        let name = fit(fb, file_name(path), r.w.saturating_sub(TEXT_PAD * 2));
        fb.text_ttf((r.x + TEXT_PAD) as i32, (r.y + 11) as i32, name, color, BODY_PX);
    }
}

fn fit<'a>(fb: &PaintBuffer, s: &'a str, max: u32) -> &'a str {
    if fb.measure_ttf(s, BODY_PX).max(0) as u32 <= max {
        return s;
    }
    let mut end = s.len();
    while end > 0 {
        end -= 1;
        while end > 0 && !s.is_char_boundary(end) {
            end -= 1;
        }
        if fb.measure_ttf(&s[..end], BODY_PX).max(0) as u32 <= max {
            break;
        }
    }
    &s[..end]
}
