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

use crate::ui::paint::{rrect, shape};
use nonos_app_skeleton::paint::PaintBuffer;

pub fn cc(fb: &mut PaintBuffer, x: u32, y: u32, s: u32, argb: u32) {
    let t = 1 + s / 16;
    let h = s.saturating_sub(s / 4);
    rrect::stroke_round(fb, x, y + s / 8, s, h, 1 + s / 6, t, argb);
    let bw = s / 4;
    let by = y + s / 2 - t / 2;
    fb.fill_rect(x + s / 5, by, bw, t, argb);
    fb.fill_rect(x + s.saturating_sub(s / 5 + bw), by, bw, t, argb);
}

pub fn pip(fb: &mut PaintBuffer, x: u32, y: u32, s: u32, argb: u32) {
    let t = 1 + s / 16;
    let h = s.saturating_sub(s / 5);
    shape::border(fb, x, y + s / 10, s, h, t, argb);
    let iw = s * 2 / 5;
    let ih = h * 2 / 5;
    let ix = x + s.saturating_sub(iw + t + 1);
    let iy = y + s / 10 + h.saturating_sub(ih + t + 1);
    fb.fill_rect(ix, iy, iw, ih, argb);
}

pub fn fullscreen(fb: &mut PaintBuffer, x: u32, y: u32, s: u32, argb: u32) {
    let t = 1 + s / 12;
    let a = 1 + s / 3;
    let r = x + s.saturating_sub(a);
    let b = y + s.saturating_sub(a);
    fb.fill_rect(x, y, a, t, argb);
    fb.fill_rect(x, y, t, a, argb);
    fb.fill_rect(r, y, a, t, argb);
    fb.fill_rect(x + s - t, y, t, a, argb);
    fb.fill_rect(x, y + s - t, a, t, argb);
    fb.fill_rect(x, b, t, a, argb);
    fb.fill_rect(r, y + s - t, a, t, argb);
    fb.fill_rect(x + s - t, b, t, a, argb);
}

pub fn clock(fb: &mut PaintBuffer, x: u32, y: u32, s: u32, argb: u32) {
    let t = 1 + s / 14;
    let r = s / 2;
    let cx = x + r;
    let cy = y + r;
    shape::ring(fb, cx, cy, r, t, argb);
    let hw = 1 + s / 16;
    fb.fill_rect(cx.saturating_sub(hw / 2), cy.saturating_sub(s / 4), hw, s / 4, argb);
    fb.fill_rect(cx, cy.saturating_sub(hw / 2), s / 4, hw, argb);
}
