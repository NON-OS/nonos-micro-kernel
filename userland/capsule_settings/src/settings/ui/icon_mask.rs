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

//! Coverage mask for one icon.
//!
//! Strokes and arcs rasterize into a shared alpha mask and blend into the
//! framebuffer once, so a join between two segments is as opaque as the middle
//! of either — drawn straight to the buffer, the overlap double-blends and reads
//! as a dark knot. Coverage is a 4x4 subpixel count against squared distances,
//! so nothing here needs a square root.

use super::icon_dist::{dist2, span, STEP, SUB};

pub const MASK_DIM: usize = 64;

pub struct Mask {
    cover: [u8; MASK_DIM * MASK_DIM],
    dim: u32,
}

impl Mask {
    pub fn new(dim: u32) -> Mask {
        Mask { cover: [0; MASK_DIM * MASK_DIM], dim: dim.min(MASK_DIM as u32) }
    }

    pub fn dim(&self) -> u32 {
        self.dim
    }

    pub fn at(&self, x: u32, y: u32) -> u8 {
        self.cover[(y * MASK_DIM as u32 + x) as usize]
    }

    pub fn segment(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, half: f32) {
        let (lo_x, hi_x) = span(x0, x1, half, self.dim);
        let (lo_y, hi_y) = span(y0, y1, half, self.dim);
        let r2 = half * half;
        for py in lo_y..hi_y {
            for px in lo_x..hi_x {
                let mut hits = 0i32;
                for sy in 0..SUB {
                    for sx in 0..SUB {
                        let fx = px as f32 + (sx as f32 + 0.5) * STEP;
                        let fy = py as f32 + (sy as f32 + 0.5) * STEP;
                        if dist2(fx, fy, x0, y0, x1, y1) <= r2 {
                            hits += 1;
                        }
                    }
                }
                if hits == 0 {
                    continue;
                }
                let a = (hits * 255 / (SUB * SUB)) as u8;
                let slot = &mut self.cover[(py * MASK_DIM as u32 + px) as usize];
                if a > *slot {
                    *slot = a;
                }
            }
        }
    }
}
