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

use nonos_app_skeleton::PaintBuffer;

use crate::browser::css::Shadow;

use super::grad::put_pixel;

// Paint a drop shadow behind the box: an offset rounded rect of the shadow
// color, the blur faked by fading concentric outlines so the edge softens
// instead of drawing a hard block.
pub(super) fn paint_shadow(fb: &mut PaintBuffer, s: &Shadow, x: i32, y: i32, w: i32, h: i32) {
    let base_a = (s.color >> 24) & 0xff;
    if base_a == 0 || w <= 0 || h <= 0 {
        return;
    }
    let blur = (s.blur as i32).clamp(0, 40);
    let ox = x + s.dx;
    let oy = y + s.dy;
    // Solid core, then softening rings out to the blur radius.
    for ring in 0..=blur {
        let a = base_a * (blur + 1 - ring) as u32 / (blur + 1) as u32;
        let color = (a << 24) | (s.color & 0x00ff_ffff);
        rect_outline(fb, ox - ring, oy - ring, w + 2 * ring, h + 2 * ring, color);
    }
}

fn rect_outline(fb: &mut PaintBuffer, x: i32, y: i32, w: i32, h: i32, color: u32) {
    if w <= 0 || h <= 0 {
        return;
    }
    for px in x..x + w {
        put_pixel(fb, px, y, color);
        put_pixel(fb, px, y + h - 1, color);
    }
    for py in y..y + h {
        put_pixel(fb, x, py, color);
        put_pixel(fb, x + w - 1, py, color);
    }
}
