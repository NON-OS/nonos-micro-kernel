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

use super::frame_rect::{frame_rect, margin, radius};
use super::metrics::{HAIRLINE_PX, LIGHT_D, LIGHT_INSET, TITLEBAR_H, TITLE_PX};
use super::palette::{FRAME_BG, FRAME_BORDER, HAIRLINE, SHADOW, TITLE_TEXT, TRANSPARENT};
use super::traffic_lights::draw_traffic_lights;
use crate::paint::{measure_ttf, PaintBuffer};

pub fn draw_frame(fb: &mut PaintBuffer, maximized: bool, title: &[u8], hover: bool) {
    let (w, h) = (fb.width, fb.height);
    let f = frame_rect(w, h, maximized);
    let r = radius(maximized);
    fb.clear(TRANSPARENT);
    fb.shadow_round(f.x, f.y, f.w, f.h, r, margin(maximized), SHADOW);
    fb.panel(f.x, f.y, f.w, f.h, r, FRAME_BG, FRAME_BORDER);
    fb.fill_rect(f.x, f.y + TITLEBAR_H, f.w, HAIRLINE_PX, HAIRLINE);
    draw_traffic_lights(fb, w, h, maximized, hover);
    draw_title(fb, w, h, maximized, title);
}

fn draw_title(fb: &mut PaintBuffer, w: u32, h: u32, maximized: bool, title: &[u8]) {
    let text = match core::str::from_utf8(title) {
        Ok(t) => t,
        Err(_) => return,
    };
    let f = frame_rect(w, h, maximized);
    let tw = measure_ttf(text, TITLE_PX).max(0) as u32;
    let floor = f.x + LIGHT_INSET * 2 + LIGHT_D * 3;
    let centred = f.x + f.w.saturating_sub(tw) / 2;
    let x = centred.max(floor);
    let y = f.y + TITLEBAR_H / 2 - (TITLE_PX as u32) / 2;
    fb.text_ttf(x as i32, y as i32, text, TITLE_TEXT, TITLE_PX);
}
