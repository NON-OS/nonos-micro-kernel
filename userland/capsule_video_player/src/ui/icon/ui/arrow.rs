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
use nonos_app_skeleton::paint::PaintBuffer;

pub fn chevron_down(fb: &mut PaintBuffer, x: u32, y: u32, s: u32, argb: u32) {
    let t = 1 + s / 12;
    let n = s / 3;
    let cy = y + s / 2 - n / 2;
    for i in 0..=n {
        fb.fill_rect(x + s / 2 - n + i, cy + i, t, t, argb);
        fb.fill_rect(x + s / 2 + i, cy + n - i, t, t, argb);
    }
}

pub fn chevron_right(fb: &mut PaintBuffer, x: u32, y: u32, s: u32, argb: u32) {
    let t = 1 + s / 12;
    let n = s / 3;
    let cx = x + s / 2 - n / 2;
    let cy = y + s / 2;
    for i in 0..=n {
        fb.fill_rect(cx + i, cy - n + i, t, t, argb);
        fb.fill_rect(cx + n - i, cy + i, t, t, argb);
    }
}

pub fn chevron_left(fb: &mut PaintBuffer, x: u32, y: u32, s: u32, argb: u32) {
    let t = 1 + s / 12;
    let n = s / 3;
    let cx = x + s / 2 - n / 2;
    let cy = y + s / 2;
    for i in 0..=n {
        fb.fill_rect(cx + n - i, cy - n + i, t, t, argb);
        fb.fill_rect(cx + i, cy + i, t, t, argb);
    }
}

pub fn back(fb: &mut PaintBuffer, x: u32, y: u32, s: u32, argb: u32) {
    let t = 1 + s / 12;
    let cy = y + s / 2;
    let a = s / 3;
    glyph::triangle_left(fb, x + s / 8, cy.saturating_sub(a), a * 2, argb);
    let hx = x + s / 8 + 3 + a;
    let hw = (x + s.saturating_sub(s / 8)).saturating_sub(hx);
    fb.fill_rect(hx, cy.saturating_sub(t / 2), hw, t, argb);
}
