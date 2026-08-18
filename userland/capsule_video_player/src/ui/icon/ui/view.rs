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

use crate::ui::paint::shape;
use nonos_app_skeleton::paint::PaintBuffer;

pub fn search(fb: &mut PaintBuffer, x: u32, y: u32, s: u32, argb: u32) {
    let t = 1 + s / 12;
    let r = s / 3;
    let cx = x + r + t;
    let cy = y + r + t;
    shape::ring(fb, cx, cy, r, t, argb);
    let n = (s + 1).saturating_sub((r + t) * 2);
    let h0 = (r * 7) / 10;
    for i in 0..n + r - h0 {
        fb.fill_rect(cx + h0 + i, cy + h0 + i, t, t, argb);
    }
}

pub fn grid(fb: &mut PaintBuffer, x: u32, y: u32, s: u32, argb: u32) {
    let g = 1 + s / 10;
    let d = s.saturating_sub(g) / 2;
    for i in 0..4 {
        fb.fill_rect(x + (i % 2) * (d + g), y + (i / 2) * (d + g), d, d, argb);
    }
}

pub fn list(fb: &mut PaintBuffer, x: u32, y: u32, s: u32, argb: u32) {
    let t = 1 + s / 12;
    let d = t + 1;
    let step = s / 4;
    let lead = d + t + 1;
    for i in 0..3 {
        let ly = y + step / 2 + i * step;
        fb.fill_rect(x, ly, d, d, argb);
        fb.fill_rect(x + lead, ly, s.saturating_sub(lead), t, argb);
    }
}

pub fn info(fb: &mut PaintBuffer, x: u32, y: u32, s: u32, argb: u32) {
    let t = 1 + s / 14;
    let r = s / 2;
    let cx = x + r;
    let cy = y + r;
    shape::ring(fb, cx, cy, r, t, argb);
    let w = 1 + s / 10;
    fb.fill_rect(cx.saturating_sub(w / 2), cy.saturating_sub(s / 4), w, w, argb);
    fb.fill_rect(cx.saturating_sub(w / 2), cy, w, s / 4, argb);
}
