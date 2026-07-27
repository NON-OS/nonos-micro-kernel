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

//! Rounded-rectangle fills painted straight into the framebuffer.

use nonos_app_skeleton::PaintBuffer;

use crate::ui::geometry::Rect;

fn corner_inset(rad: u32, dy: u32) -> u32 {
    let lim = (rad * rad).saturating_sub(dy * dy);
    let mut k = 0;
    while k < rad && (rad - k) * (rad - k) > lim {
        k += 1;
    }
    k
}

pub fn fill_round(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, h: u32, rad: u32, argb: u32) {
    let rad = rad.min(h / 2).min(w / 2);
    for row in 0..h {
        let dy = if row < rad {
            rad - row
        } else if row + rad >= h {
            row + rad + 1 - h
        } else {
            0
        };
        let k = corner_inset(rad, dy);
        fb.fill_rect(x + k, y + row, w.saturating_sub(k * 2), 1, argb);
    }
}

pub fn chip(fb: &mut PaintBuffer, r: &Rect, rad: u32, fill: u32, border: u32) {
    fill_round(fb, r.x, r.y, r.w, r.h, rad, border);
    fill_round(
        fb,
        r.x + 1,
        r.y + 1,
        r.w.saturating_sub(2),
        r.h.saturating_sub(2),
        rad.saturating_sub(1),
        fill,
    );
}
