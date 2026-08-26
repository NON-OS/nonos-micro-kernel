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
    fn cov_px(&mut self, x: i32, y: i32, argb: u32, cov: u32) {
        if x < 0 || y < 0 || cov == 0 {
            return;
        }
        let a = ((argb >> 24) & 0xFF) * cov / 256;
        if a == 0 {
            return;
        }
        self.blend_px(x as u32, y as u32, (a << 24) | (argb & 0x00FF_FFFF));
    }

    pub fn line_aa(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, argb: u32) {
        let steep = (y1 - y0).abs() > (x1 - x0).abs();
        let (ax, ay, bx, by) = if steep { (y0, x0, y1, x1) } else { (x0, y0, x1, y1) };
        let (ax, ay, bx, by) = if ax > bx { (bx, by, ax, ay) } else { (ax, ay, bx, by) };
        let dx = bx - ax;
        if dx == 0 {
            self.cov_px(x0, y0, argb, 256);
            return;
        }
        let grad = ((by - ay) * 256) / dx;
        let mut inter = ay * 256;
        for x in ax..=bx {
            let base = inter >> 8;
            let frac = (inter & 0xFF) as u32;
            let (p0, p1) = if steep { (base, x) } else { (x, base) };
            let (q0, q1) = if steep { (base + 1, x) } else { (x, base + 1) };
            self.cov_px(p0, p1, argb, 256 - frac);
            self.cov_px(q0, q1, argb, frac);
            inter += grad;
        }
    }

    pub fn polyline_aa(&mut self, pts: &[(i32, i32)], argb: u32) {
        if pts.len() < 2 {
            return;
        }
        let mut i = 1;
        while i < pts.len() {
            let (x0, y0) = pts[i - 1];
            let (x1, y1) = pts[i];
            self.line_aa(x0, y0, x1, y1, argb);
            i += 1;
        }
    }
}
