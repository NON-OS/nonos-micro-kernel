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

pub fn play(fb: &mut PaintBuffer, x: u32, y: u32, s: u32, argb: u32) {
    glyph::triangle_right(fb, x + s / 3, y + s / 8, s.saturating_sub(s / 4), argb);
}

pub fn pause(fb: &mut PaintBuffer, x: u32, y: u32, s: u32, argb: u32) {
    let w = 1 + s / 5;
    let gap = s / 5;
    let h = s.saturating_sub(s / 4);
    let ox = s.saturating_sub(w * 2 + gap) / 2;
    glyph::pause(fb, x + ox, y + s / 8, w, h, gap, argb);
}

pub fn prev(fb: &mut PaintBuffer, x: u32, y: u32, s: u32, argb: u32) {
    let h = s.saturating_sub(s / 3);
    let t = 1 + s / 10;
    fb.fill_rect(x + s / 8, y + s / 6, t, h, argb);
    glyph::triangle_left(fb, x + s / 8 + t + 1, y + s / 6, h, argb);
}

pub fn next(fb: &mut PaintBuffer, x: u32, y: u32, s: u32, argb: u32) {
    let h = s.saturating_sub(s / 3);
    let t = 1 + s / 10;
    let tx = s.saturating_sub(t + s / 8 + h / 2 + 4);
    fb.fill_rect(x + s.saturating_sub(t + s / 8), y + s / 6, t, h, argb);
    glyph::triangle_right(fb, x + tx, y + s / 6, h, argb);
}
