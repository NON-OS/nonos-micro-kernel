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
    pub fn fill_round(&mut self, x: u32, y: u32, w: u32, h: u32, r: u32, argb: u32) {
        if w == 0 || h == 0 {
            return;
        }
        let r = radius::clamp_radius(w, h, r);
        if r == 0 {
            self.blend_rect(x, y, w, h, argb);
            return;
        }
        let a = (argb >> 24) & 0xFF;
        let rgb = argb & 0x00FF_FFFF;
        for row in 0..h {
            if row >= r && row < h - r {
                self.blend_rect(x, y + row, w, 1, argb);
                continue;
            }
            for col in 0..w {
                let cov = radius::coverage(col, row, w, h, r);
                if cov == 0 {
                    continue;
                }
                let ea = a * cov / 255;
                self.blend_px(x + col, y + row, (ea << 24) | rgb);
            }
        }
    }
}
