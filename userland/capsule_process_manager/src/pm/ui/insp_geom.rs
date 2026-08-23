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

use super::metrics::{INSPECTOR_PAD, INSPECTOR_W, INSP_BTN_BOTTOM, INSP_BTN_GAP, INSP_BTN_H};

pub fn pane_x(fb_w: u32) -> u32 {
    fb_w.saturating_sub(INSPECTOR_W)
}

pub fn content_x(fb_w: u32) -> u32 {
    pane_x(fb_w) + INSPECTOR_PAD
}

pub fn content_w() -> u32 {
    INSPECTOR_W.saturating_sub(INSPECTOR_PAD * 2)
}

// The two actions stack up from the bottom of the pane, clear of the status
// strip: index 0 is End Process, index 1 is Force Quit. The painter and the hit
// test both read the rect from here rather than each deriving one, which is the
// only reason a click cannot land on the button next to the one it looks at.
pub fn btn(fb_w: u32, fb_h: u32, index: usize) -> (u32, u32, u32, u32) {
    let last = fb_h.saturating_sub(INSP_BTN_BOTTOM + INSP_BTN_H);
    let lift = if index == 0 { INSP_BTN_H + INSP_BTN_GAP } else { 0 };
    (content_x(fb_w), last.saturating_sub(lift), content_w(), INSP_BTN_H)
}

pub fn btn_at(fb_w: u32, fb_h: u32, x: i32, y: i32) -> Option<usize> {
    (0..2).find(|i| {
        let (bx, by, bw, bh) = btn(fb_w, fb_h, *i);
        x >= bx as i32 && x < (bx + bw) as i32 && y >= by as i32 && y < (by + bh) as i32
    })
}
