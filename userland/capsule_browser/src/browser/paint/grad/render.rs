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

use nonos_app_skeleton::PaintBuffer;

use super::parse::Linear;
use super::stops::color_at;
use super::trig::{cos, sin};

// Fill the box [x, y, w, h] with a linear gradient, source-over compositing so
// a semi-transparent overlay shows the content beneath it. The gradient line
// runs along the CSS angle; a pixel's projection onto it gives its stop t.
pub(super) fn fill_linear(fb: &mut PaintBuffer, g: &Linear, x: i32, y: i32, w: i32, h: i32) {
    if w <= 0 || h <= 0 {
        return;
    }
    let rad = g.angle * core::f32::consts::PI / 180.0;
    // Axis unit vector: 0deg points up, angle increases clockwise.
    let (ux, uy) = (sin(rad), -cos(rad));
    // Project the four corners to normalize the gradient line to 0..1.
    let mut lo = f32::MAX;
    let mut hi = f32::MIN;
    for &(cx, cy) in &[(0.0, 0.0), (w as f32, 0.0), (0.0, h as f32), (w as f32, h as f32)] {
        let p = cx * ux + cy * uy;
        lo = lo.min(p);
        hi = hi.max(p);
    }
    let span = (hi - lo).max(1.0);
    for py in 0..h {
        for px in 0..w {
            let t = ((px as f32 * ux + py as f32 * uy) - lo) / span;
            put_pixel(fb, x + px, y + py, color_at(&g.stops, t));
        }
    }
}

// Source-over one gradient sample onto the framebuffer.
pub(super) fn put_pixel(fb: &mut PaintBuffer, x: i32, y: i32, argb: u32) {
    if x < 0 || y < 0 || x as u32 >= fb.width || y as u32 >= fb.height {
        return;
    }
    let idx = y as usize * fb.stride_words as usize + x as usize;
    let Some(dst) = fb.pixels.get(idx).copied() else { return };
    let a = (argb >> 24) & 0xff;
    if a == 0 {
        return;
    }
    if a == 255 {
        fb.pixels[idx] = 0xff00_0000 | (argb & 0x00ff_ffff);
        return;
    }
    let mix = |s: u32, d: u32| (s * a + d * (255 - a)) / 255;
    let r = mix((argb >> 16) & 0xff, (dst >> 16) & 0xff);
    let gr = mix((argb >> 8) & 0xff, (dst >> 8) & 0xff);
    let b = mix(argb & 0xff, dst & 0xff);
    fb.pixels[idx] = 0xff00_0000 | (r << 16) | (gr << 8) | b;
}
