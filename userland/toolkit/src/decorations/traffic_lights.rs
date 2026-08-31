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

use super::frame_rect::light_rect;
use super::metrics::LIGHT_D;
use super::palette::{LIGHT_CLOSE, LIGHT_GLYPH, LIGHT_MAXIMIZE, LIGHT_MINIMIZE};
use crate::paint::PaintBuffer;

const LIGHTS: [u32; 3] = [LIGHT_CLOSE, LIGHT_MINIMIZE, LIGHT_MAXIMIZE];

pub fn draw_traffic_lights(fb: &mut PaintBuffer, w: u32, h: u32, maximized: bool, hover: bool) {
    for (i, argb) in LIGHTS.iter().enumerate() {
        let r = light_rect(i as u32, w, h, maximized);
        let rad = LIGHT_D / 2;
        fb.circle(r.x + rad, r.y + rad, rad, *argb);
        if hover {
            draw_glyph(fb, i as u32, r.x + rad, r.y + rad);
        }
    }
}

fn draw_glyph(fb: &mut PaintBuffer, i: u32, cx: u32, cy: u32) {
    let x = cx as i32;
    let y = cy as i32;
    match i {
        0 => {
            fb.line(x - 2, y - 2, x + 2, y + 2, LIGHT_GLYPH);
            fb.line(x + 2, y - 2, x - 2, y + 2, LIGHT_GLYPH);
        }
        1 => fb.line(x - 3, y, x + 3, y, LIGHT_GLYPH),
        _ => {
            fb.line(x - 2, y - 2, x + 2, y - 2, LIGHT_GLYPH);
            fb.line(x - 2, y - 2, x - 2, y + 2, LIGHT_GLYPH);
            fb.line(x - 2, y + 2, x + 2, y + 2, LIGHT_GLYPH);
            fb.line(x + 2, y - 2, x + 2, y + 2, LIGHT_GLYPH);
        }
    }
}
