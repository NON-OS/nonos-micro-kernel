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

use super::rrect;
use nonos_app_skeleton::paint::PaintBuffer;

pub fn circle(fb: &mut PaintBuffer, cx: u32, cy: u32, r: u32, argb: u32) {
    let d = r * 2;
    rrect::fill_round(fb, cx.saturating_sub(r), cy.saturating_sub(r), d, d, r, argb);
}

pub fn ring(fb: &mut PaintBuffer, cx: u32, cy: u32, r: u32, t: u32, argb: u32) {
    let d = r * 2;
    rrect::stroke_round(fb, cx.saturating_sub(r), cy.saturating_sub(r), d, d, r, t, argb);
}

pub fn hline(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, argb: u32) {
    fb.fill_rect(x, y, w, 1, argb);
}

pub fn vline(fb: &mut PaintBuffer, x: u32, y: u32, h: u32, argb: u32) {
    fb.fill_rect(x, y, 1, h, argb);
}

pub fn border(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, h: u32, t: u32, argb: u32) {
    if t == 0 || w == 0 || h == 0 {
        return;
    }
    let t = if t * 2 > h { h } else { t };
    fb.fill_rect(x, y, w, t, argb);
    fb.fill_rect(x, y + h - t, w, t, argb);
    let mid = h.saturating_sub(t * 2);
    let side = if t * 2 > w { w } else { t };
    if mid > 0 {
        fb.fill_rect(x, y + t, side, mid, argb);
        fb.fill_rect(x + w - side, y + t, side, mid, argb);
    }
}

pub fn hex(fb: &mut PaintBuffer, cx: u32, cy: u32, r: u32, argb: u32) {
    if r == 0 {
        return;
    }
    let flat = (r * 866) / 1000;
    let straight = r / 2;
    let taper = r - straight;
    let top = cy.saturating_sub(r);
    for row in 0..=r * 2 {
        let dy = if row > r { row - r } else { r - row };
        let half = if dy <= straight { flat } else { flat * (r - dy) / taper };
        if half > 0 {
            fb.fill_rect(cx.saturating_sub(half), top + row, half * 2, 1, argb);
        }
    }
}
