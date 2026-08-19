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


use super::mixer::lerp_argb;
use super::PaintBuffer;

impl<'a> PaintBuffer<'a> {
    pub fn gradient_v(&mut self, x: u32, y: u32, w: u32, h: u32, top: u32, bottom: u32) {
        if w == 0 || h == 0 {
            return;
        }
        for row in 0..h {
            let t = if h == 1 { 0 } else { row * 255 / (h - 1) };
            self.blend_rect(x, y + row, w, 1, lerp_argb(top, bottom, t));
        }
    }

    pub fn gradient_h(&mut self, x: u32, y: u32, w: u32, h: u32, left: u32, right: u32) {
        if w == 0 || h == 0 {
            return;
        }
        for col in 0..w {
            let t = if w == 1 { 0 } else { col * 255 / (w - 1) };
            self.blend_rect(x + col, y, 1, h, lerp_argb(left, right, t));
        }
    }
}
