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
use crate::ui::paint::rrect;
use nonos_app_skeleton::paint::PaintBuffer;

pub fn shuffle(fb: &mut PaintBuffer, x: u32, y: u32, s: u32, argb: u32) {
    let t = 1 + s / 12;
    let n = s.saturating_sub(t);
    for i in 0..n {
        fb.fill_rect(x + i, y + i, t, t, argb);
        fb.fill_rect(x + i, y + n - i, t, t, argb);
    }
    let a = 2 + s / 8;
    fb.fill_rect(x + s - a, y, a, t, argb);
    fb.fill_rect(x + s - t, y, t, a, argb);
    fb.fill_rect(x + s - a, y + s - t, a, t, argb);
    fb.fill_rect(x + s - t, y + s.saturating_sub(a), t, a, argb);
}

pub fn repeat(fb: &mut PaintBuffer, x: u32, y: u32, s: u32, argb: u32) {
    let t = 1 + s / 12;
    let h = s.saturating_sub(s / 4);
    rrect::stroke_round(fb, x, y + s / 8, s, h, 1 + s / 5, t, argb);
    let a = 2 + s / 6;
    let ty = (y + s / 8 + t / 2).saturating_sub(a / 2);
    glyph::triangle_right(fb, x + s.saturating_sub(3 + a / 2), ty, a, argb);
}
