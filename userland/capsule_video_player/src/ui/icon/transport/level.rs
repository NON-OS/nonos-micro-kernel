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

use nonos_app_skeleton::paint::PaintBuffer;

fn speaker(fb: &mut PaintBuffer, x: u32, y: u32, s: u32, argb: u32) {
    let c = s / 2;
    let bw = 1 + s / 8;
    let bh = 1 + s / 4;
    let bx = x + s / 10;
    fb.fill_rect(bx, y + c.saturating_sub(bh / 2), bw, bh, argb);
    let cw = 1 + s / 5;
    for col in 0..cw {
        let hh = bh / 2 + (col + 1) * c.saturating_sub(bh / 2 + c / 8) / cw;
        fb.fill_rect(bx + bw + col, y + c.saturating_sub(hh), 1, hh * 2, argb);
    }
}

fn wave(fb: &mut PaintBuffer, x: u32, cy: u32, h: u32, t: u32, argb: u32) {
    fb.fill_rect(x, cy.saturating_sub(h / 2), t, h, argb);
    fb.fill_rect(x.saturating_sub(t), cy.saturating_sub(h / 2 + t), t, t, argb);
    fb.fill_rect(x.saturating_sub(t), cy + h / 2, t, t, argb);
}

pub fn volume(fb: &mut PaintBuffer, x: u32, y: u32, s: u32, argb: u32) {
    speaker(fb, x, y, s, argb);
    let t = 1 + s / 16;
    let cy = y + s / 2;
    wave(fb, x + s / 2 + s / 10, cy, s / 3, t, argb);
    wave(fb, x + s.saturating_sub(t * 2), cy, s.saturating_sub(s / 4 + t), t, argb);
}

pub fn mute(fb: &mut PaintBuffer, x: u32, y: u32, s: u32, argb: u32) {
    speaker(fb, x, y, s, argb);
    let t = 1 + s / 16;
    let n = s / 3;
    let bx = x + s.saturating_sub(n + t);
    let by = y + s / 2 - n / 2;
    for i in 0..n {
        fb.fill_rect(bx + i, by + i, t, t, argb);
        fb.fill_rect(bx + i, by + n - i - 1, t, t, argb);
    }
}
