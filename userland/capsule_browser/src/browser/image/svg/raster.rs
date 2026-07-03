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

use alloc::vec;
use alloc::vec::Vec;

// Working canvas at 2x the output size; the box filter on the way out is
// the antialiasing.
pub(super) const SS: u32 = 2;

pub(super) struct Raster {
    pub w: u32,
    pub h: u32,
    pub px: Vec<u32>,
}

impl Raster {
    pub fn new(out_w: u32, out_h: u32) -> Self {
        let (w, h) = (out_w * SS, out_h * SS);
        Raster { w, h, px: vec![0; (w * h) as usize] }
    }

    // Source-over composite of an ARGB color onto one pixel.
    pub fn blend(&mut self, x: i32, y: i32, color: u32) {
        if x < 0 || y < 0 || x >= self.w as i32 || y >= self.h as i32 {
            return;
        }
        let idx = y as usize * self.w as usize + x as usize;
        let Some(dst) = self.px.get_mut(idx) else { return };
        let sa = color >> 24;
        if sa == 0xFF {
            *dst = color;
            return;
        }
        if sa == 0 {
            return;
        }
        let inv = 255 - sa;
        let ch = |s: u32, d: u32| (s * sa + d * inv) / 255;
        let da = (*dst) >> 24;
        let a = sa + da * inv / 255;
        let r = ch((color >> 16) & 0xFF, (*dst >> 16) & 0xFF);
        let g = ch((color >> 8) & 0xFF, (*dst >> 8) & 0xFF);
        let b = ch(color & 0xFF, *dst & 0xFF);
        *dst = a << 24 | r << 16 | g << 8 | b;
    }
}
