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


use super::PaintBuffer;

impl<'a> PaintBuffer<'a> {
    pub fn line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, argb: u32) {
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;
        let mut x = x0;
        let mut y = y0;
        loop {
            if x >= 0 && y >= 0 {
                self.blend_px(x as u32, y as u32, argb);
            }
            if x == x1 && y == y1 {
                return;
            }
            let e2 = err * 2;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }

    pub fn polyline(&mut self, pts: &[(i32, i32)], argb: u32) {
        if pts.len() < 2 {
            return;
        }
        let mut i = 1;
        while i < pts.len() {
            let (x0, y0) = pts[i - 1];
            let (x1, y1) = pts[i];
            self.line(x0, y0, x1, y1, argb);
            i += 1;
        }
    }
}
