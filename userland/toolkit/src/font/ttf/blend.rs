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

// Composite one coverage sample over an ARGB8888 pixel. Coverage is the
// glyph's alpha at this pixel; the surface stays opaque.
pub(super) fn blend(
    buf: &mut [u32],
    stride: usize,
    w: u32,
    h: u32,
    x: i32,
    y: i32,
    argb: u32,
    a: f32,
) {
    if x < 0 || y < 0 || x >= w as i32 || y >= h as i32 {
        return;
    }
    let a = if a < 0.0 {
        0.0
    } else if a > 1.0 {
        1.0
    } else {
        a
    };
    let idx = y as usize * stride + x as usize;
    if idx >= buf.len() {
        return;
    }
    // The color's own alpha scales the coverage, so translucent text fades
    // instead of ignoring its opacity.
    let a = a * ((argb >> 24) & 0xff) as f32 / 255.0;
    let dst = buf[idx];
    let (sr, sg, sb) = ((argb >> 16) & 0xff, (argb >> 8) & 0xff, argb & 0xff);
    let (dr, dg, db) = ((dst >> 16) & 0xff, (dst >> 8) & 0xff, dst & 0xff);
    let mix = |s: u32, d: u32| -> u32 { (s as f32 * a + d as f32 * (1.0 - a) + 0.5) as u32 };
    buf[idx] = 0xff00_0000 | (mix(sr, dr) << 16) | (mix(sg, dg) << 8) | mix(sb, db);
}
