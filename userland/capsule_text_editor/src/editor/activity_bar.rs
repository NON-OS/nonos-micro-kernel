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

//! The left activity bar: stacked Editor, Home and Settings buttons. Each is a
//! small glyph with an accent rail on the row whose screen is showing.

use nonos_app_skeleton::PaintBuffer;

use super::layout::{ACTIVITY_W, TITLEBAR_H};
use super::screen::Screen;
use super::theme;

const ROW_TOP: u32 = TITLEBAR_H + 12;
const ROW_PITCH: u32 = 44;
const ROW_COUNT: u32 = 3;

pub(super) fn row_screen(row: usize) -> Screen {
    match row {
        0 => Screen::Editor,
        1 => Screen::Home,
        _ => Screen::Settings,
    }
}

pub(super) fn paint_activity(fb: &mut PaintBuffer, height: u32, screen: Screen, sidebar_open: bool) {
    let th = theme::active();
    fb.fill_rect(0, 0, ACTIVITY_W, height, th.activity_bg);
    for row in 0..ROW_COUNT {
        let top = ROW_TOP + row * ROW_PITCH;
        let selected = row_screen(row as usize) == screen;
        if selected {
            fb.fill_rect(0, top, 2, 30, th.accent);
        }
        let lit = selected && (row != 0 || sidebar_open);
        let color = if lit { th.icon_active } else { th.icon };
        paint_row_icon(fb, row, top + 6, color);
    }
}

fn paint_row_icon(fb: &mut PaintBuffer, row: u32, y: u32, color: u32) {
    let ix = 15;
    match row {
        0 => {
            for k in 0..3 {
                fb.fill_rect(ix, y + k * 7, 16, 2, color);
            }
        }
        1 => {
            fb.fill_rect(ix, y, 16, 2, color);
            fb.fill_rect(ix, y, 2, 16, color);
            fb.fill_rect(ix + 14, y, 2, 16, color);
            fb.fill_rect(ix, y + 14, 16, 2, color);
        }
        _ => {
            for k in 0..2 {
                fb.fill_rect(ix, y + 3 + k * 9, 16, 2, color);
                fb.fill_rect(ix + 2 + k * 8, y + 1 + k * 9, 4, 6, color);
            }
        }
    }
}

pub(super) fn activity_hit(y: i32) -> Option<usize> {
    if y < 0 || (y as u32) < ROW_TOP {
        return None;
    }
    let row = ((y as u32) - ROW_TOP) / ROW_PITCH;
    if row < ROW_COUNT {
        Some(row as usize)
    } else {
        None
    }
}
