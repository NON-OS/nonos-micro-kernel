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

use super::radius;
use nonos_app_skeleton::paint::PaintBuffer;

pub fn fill_round(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, h: u32, r: u32, argb: u32) {
    let r = radius::clamp_radius(w, h, r);
    for row in 0..h {
        let d = radius::inset(row, h, r);
        let span = w.saturating_sub(d * 2);
        if span > 0 {
            fb.fill_rect(x + d, y + row, span, 1, argb);
        }
    }
}

pub fn stroke_round(
    fb: &mut PaintBuffer,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
    r: u32,
    t: u32,
    argb: u32,
) {
    if t == 0 || w == 0 || h == 0 {
        return;
    }
    if w <= t * 2 || h <= t * 2 {
        fill_round(fb, x, y, w, h, r, argb);
        return;
    }
    let r = radius::clamp_radius(w, h, r);
    let iw = w - t * 2;
    let ih = h - t * 2;
    let ir = radius::clamp_radius(iw, ih, r.saturating_sub(t));
    for row in 0..h {
        let d = radius::inset(row, h, r);
        let span = w.saturating_sub(d * 2);
        if span == 0 {
            continue;
        }
        let left = x + d;
        if row < t || row >= h - t {
            fb.fill_rect(left, y + row, span, 1, argb);
            continue;
        }
        let di = radius::inset(row - t, ih, ir);
        let ileft = x + t + di;
        let iright = x + t + iw - di;
        fb.fill_rect(left, y + row, ileft.saturating_sub(left), 1, argb);
        fb.fill_rect(iright, y + row, (left + span).saturating_sub(iright), 1, argb);
    }
}

pub fn panel(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, h: u32, r: u32, fill: u32, border: u32) {
    fill_round(fb, x, y, w, h, r, fill);
    stroke_round(fb, x, y, w, h, r, 1, border);
}
