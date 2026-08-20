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


use super::radius;
use super::PaintBuffer;

impl<'a> PaintBuffer<'a> {
    pub fn stroke_round(&mut self, x: u32, y: u32, w: u32, h: u32, r: u32, t: u32, argb: u32) {
        if t == 0 || w == 0 || h == 0 {
            return;
        }
        if w <= t * 2 || h <= t * 2 {
            self.fill_round(x, y, w, h, r, argb);
            return;
        }
        let r = radius::clamp_radius(w, h, r);
        let iw = w - t * 2;
        let ih = h - t * 2;
        let ir = radius::clamp_radius(iw, ih, r.saturating_sub(t));
        let a = (argb >> 24) & 0xFF;
        let rgb = argb & 0x00FF_FFFF;
        for row in 0..h {
            if row >= r && row < h - r {
                self.blend_rect(x, y + row, t, 1, argb);
                self.blend_rect(x + w - t, y + row, t, 1, argb);
                continue;
            }
            for col in 0..w {
                let outer = radius::coverage(col, row, w, h, r);
                if outer == 0 {
                    continue;
                }
                let inner = if row >= t && row < h - t && col >= t && col < w - t {
                    radius::coverage(col - t, row - t, iw, ih, ir)
                } else {
                    0
                };
                let cov = outer.saturating_sub(inner);
                if cov == 0 {
                    continue;
                }
                self.blend_px(x + col, y + row, ((a * cov / 255) << 24) | rgb);
            }
        }
    }
}
