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

use super::buffer::PaintBuffer;

impl<'a> PaintBuffer<'a> {
    /// Alpha-blit a source RGBA8 image (bytes in R,G,B,A order, row-major),
    /// area-averaged down into the destination rect and composited over the
    /// pixels already in the buffer. This lets a capsule render a real,
    /// anti-aliased icon asset at any size instead of a hand-coded bitmap.
    ///
    /// Each destination pixel averages the whole source footprint it covers
    /// (alpha-weighted colour, mean coverage), which is what gives crisp edges
    /// when a high-resolution asset is scaled down to a UI size.
    pub fn blit_rgba8_scaled(
        &mut self,
        dx: u32,
        dy: u32,
        dw: u32,
        dh: u32,
        rgba: &[u8],
        sw: u32,
        sh: u32,
    ) {
        if dw == 0 || dh == 0 || sw == 0 || sh == 0 {
            return;
        }
        if rgba.len() < (sw as usize) * (sh as usize) * 4 {
            return;
        }
        let stride = self.stride_words as usize;
        for row in 0..dh {
            let py = dy + row;
            if py >= self.height {
                break;
            }
            let sy0 = row * sh / dh;
            let sy1 = core::cmp::max(sy0 + 1, (row + 1) * sh / dh).min(sh);
            for col in 0..dw {
                let px = dx + col;
                if px >= self.width {
                    break;
                }
                let sx0 = col * sw / dw;
                let sx1 = core::cmp::max(sx0 + 1, (col + 1) * sw / dw).min(sw);

                // Alpha-weighted average over the source footprint.
                let (mut sr, mut sg, mut sb, mut sa, mut cnt) = (0u32, 0u32, 0u32, 0u32, 0u32);
                for syy in sy0..sy1 {
                    let base = (syy as usize * sw as usize) * 4;
                    for sxx in sx0..sx1 {
                        let i = base + sxx as usize * 4;
                        let a = rgba[i + 3] as u32;
                        sr += rgba[i] as u32 * a;
                        sg += rgba[i + 1] as u32 * a;
                        sb += rgba[i + 2] as u32 * a;
                        sa += a;
                        cnt += 1;
                    }
                }
                if cnt == 0 || sa == 0 {
                    continue;
                }
                let a = sa / cnt; // mean coverage 0..255
                let r = (sr / sa).min(255);
                let g = (sg / sa).min(255);
                let b = (sb / sa).min(255);

                let idx = py as usize * stride + px as usize;
                if idx >= self.pixels.len() {
                    continue;
                }
                let out = if a >= 255 {
                    (r << 16) | (g << 8) | b
                } else {
                    let d = self.pixels[idx];
                    let dr = (d >> 16) & 0xFF;
                    let dg = (d >> 8) & 0xFF;
                    let db = d & 0xFF;
                    let ia = 255 - a;
                    let or = (r * a + dr * ia) / 255;
                    let og = (g * a + dg * ia) / 255;
                    let ob = (b * a + db * ia) / 255;
                    (or << 16) | (og << 8) | ob
                };
                self.pixels[idx] = 0xFF00_0000 | out;
            }
        }
    }
}
