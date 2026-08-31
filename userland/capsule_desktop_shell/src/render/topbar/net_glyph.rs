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

//! Three concentric signal arcs over a dot, dimmer still when the DHCP lease
//! is not bound.

use crate::render::palette;
use crate::render::surface::surface;
use crate::render::ui_font::scale;
use crate::state::Context;
use nonos_toolkit::paint::PaintBuffer;

const ON: u32 = palette::TEXT_DIM;
const OFF: u32 = palette::TEXT_MUTED;

pub(super) fn net_glyph(ctx: &Context, x: u32, y: u32, online: bool) {
    let s = scale();
    let color = if online { ON } else { OFF };
    let (cx, cy) = ((x + 8 * s) as i32, (y + 11 * s) as i32);
    let mut fb = surface(ctx);

    for r in [10 * s, 7 * s, 3 * s] {
        arc(&mut fb, cx, cy, r as i32, s.max(1) as i32, color);
    }
    fb.circle(cx as u32, cy.max(0) as u32, s, color);
}

// The upper 90 degrees of a circle, walked by x so the span stays gap-free at
// the shallow end where these arcs live.
fn arc(fb: &mut PaintBuffer<'_>, cx: i32, cy: i32, r: i32, t: i32, argb: u32) {
    let span = r * 7 / 10;
    for dx in -span..=span {
        let dy = isqrt((r * r - dx * dx) as u32) as i32;
        for k in 0..t.max(1) {
            let (px, py) = (cx + dx, cy - dy + k);
            if px >= 0 && py >= 0 {
                fb.blend_px(px as u32, py as u32, argb);
            }
        }
    }
}

fn isqrt(v: u32) -> u32 {
    let mut r = 0u32;
    while (r + 1) * (r + 1) <= v {
        r += 1;
    }
    r
}
