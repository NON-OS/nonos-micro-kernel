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

use crate::ui::glyph;
use crate::ui::paint::{rrect, shape};
use nonos_app_skeleton::paint::PaintBuffer;

pub fn home(fb: &mut PaintBuffer, x: u32, y: u32, s: u32, argb: u32) {
    let roof = s / 2;
    for row in 0..roof {
        let w = (row + 1) * 2;
        fb.fill_rect(x + roof.saturating_sub(row + 1), y + row, w, 1, argb);
    }
    let bw = s.saturating_sub(s / 4);
    fb.fill_rect(x + s / 8, y + roof, bw, s.saturating_sub(roof), argb);
}

pub fn library(fb: &mut PaintBuffer, x: u32, y: u32, s: u32, argb: u32) {
    let c = s / 2;
    let d = c.saturating_sub(1 + s / 12);
    for i in 0..4 {
        fb.fill_rect(x + (i % 2) * c, y + (i / 2) * c, d, d, argb);
    }
}

pub fn playlist(fb: &mut PaintBuffer, x: u32, y: u32, s: u32, argb: u32) {
    let t = 1 + s / 20;
    let th = s / 3;
    let w = s.saturating_sub(th / 2 + 4);
    for i in 0..3 {
        fb.fill_rect(x, y + s / 6 + i * (s / 4), w, t, argb);
    }
    glyph::triangle_right(fb, x + s.saturating_sub(3 + th / 2), y + s / 3, th, argb);
}

pub fn files(fb: &mut PaintBuffer, x: u32, y: u32, s: u32, argb: u32) {
    let top = s / 5;
    let lip = 1 + s / 10;
    fb.fill_rect(x, y + top, s / 2, lip, argb);
    let bh = s.saturating_sub(top + lip + top / 2);
    rrect::fill_round(fb, x, y + top + lip, s, bh, 1 + s / 12, argb);
}

pub fn gear(fb: &mut PaintBuffer, x: u32, y: u32, s: u32, argb: u32) {
    let c = s / 2;
    let sl = 1 + s / 8;
    let r = c.saturating_sub(sl);
    let sw = 1 + s / 10;
    let o = sw / 2;
    shape::ring(fb, x + c, y + c, r, 1 + s / 12, argb);
    fb.fill_rect(x + c - o, y, sw, sl, argb);
    fb.fill_rect(x + c - o, y + c + r, sw, sl, argb);
    fb.fill_rect(x, y + c - o, sl, sw, argb);
    fb.fill_rect(x + c + r, y + c - o, sl, sw, argb);
    let d = (c * 3) / 4;
    let lo = c.saturating_sub(d + o);
    fb.fill_rect(x + c + d - o, y + c + d - o, sw, sw, argb);
    fb.fill_rect(x + c + d - o, y + lo, sw, sw, argb);
    fb.fill_rect(x + lo, y + c + d - o, sw, sw, argb);
    fb.fill_rect(x + lo, y + lo, sw, sw, argb);
}

pub fn video(fb: &mut PaintBuffer, x: u32, y: u32, s: u32, argb: u32) {
    let h = s.saturating_sub(s / 4);
    rrect::stroke_round(fb, x, y + s / 8, s, h, 1 + s / 6, 1 + s / 16, argb);
    glyph::triangle_right(fb, x + s / 2 - s / 8, y + s / 2 - s / 6, s / 3, argb);
}
