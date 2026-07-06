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

use super::store::Decoded;

// Scale-blit `img` to fit the box at (x, y, box_w, box_h) preserving aspect
// ratio and centering. Bilinear sampling so a small icon scaled up stays smooth
// instead of blocky; alpha composited over the existing surface so transparent
// PNGs read correctly.
pub fn blit_into(fb: &mut PaintBuffer, img: &Decoded, x: u32, y: u32, box_w: u32, box_h: u32) {
    if img.w == 0 || img.h == 0 || box_w == 0 || box_h == 0 {
        return;
    }
    let fit_w = box_w as u64 * img.h as u64;
    let fit_h = box_h as u64 * img.w as u64;
    let (dst_w, dst_h) = if fit_w <= fit_h {
        (box_w, ((box_w as u64 * img.h as u64) / img.w as u64) as u32)
    } else {
        (((box_h as u64 * img.w as u64) / img.h as u64) as u32, box_h)
    };
    if dst_w == 0 || dst_h == 0 {
        return;
    }
    let ox = x + (box_w - dst_w) / 2;
    let oy = y + (box_h - dst_h) / 2;
    for dy in 0..dst_h {
        let gy = (dy as u64 * img.h as u64 * 256 / dst_h as u64) as u32;
        for dx in 0..dst_w {
            let gx = (dx as u64 * img.w as u64 * 256 / dst_w as u64) as u32;
            put(fb, ox + dx, oy + dy, sample(img, gx, gy));
        }
    }
}

// Bilinear sample of `img` at the 8.8 fixed-point source coordinate (gx, gy),
// blending the four surrounding texels by the fractional part.
fn sample(img: &Decoded, gx: u32, gy: u32) -> u32 {
    let (w, h) = (img.w, img.h);
    let x0 = (gx >> 8).min(w - 1);
    let y0 = (gy >> 8).min(h - 1);
    let x1 = (x0 + 1).min(w - 1);
    let y1 = (y0 + 1).min(h - 1);
    let fx = gx & 0xff;
    let fy = gy & 0xff;
    let tex = |xx: u32, yy: u32| img.px[(yy * w + xx) as usize];
    let (c00, c10, c01, c11) = (tex(x0, y0), tex(x1, y0), tex(x0, y1), tex(x1, y1));
    let chan = |sh: u32| -> u32 {
        let a = (c00 >> sh) & 0xff;
        let b = (c10 >> sh) & 0xff;
        let c = (c01 >> sh) & 0xff;
        let d = (c11 >> sh) & 0xff;
        let top = a * (256 - fx) + b * fx;
        let bot = c * (256 - fx) + d * fx;
        ((top * (256 - fy) + bot * fy) >> 16) & 0xff
    };
    (chan(24) << 24) | (chan(16) << 16) | (chan(8) << 8) | chan(0)
}

fn put(fb: &mut PaintBuffer, x: u32, y: u32, argb: u32) {
    if x >= fb.width || y >= fb.height {
        return;
    }
    let idx = y as usize * fb.stride_words as usize + x as usize;
    if idx >= fb.pixels.len() {
        return;
    }
    let a = (argb >> 24) & 0xff;
    if a == 0 {
        return;
    }
    if a == 255 {
        fb.pixels[idx] = 0xff00_0000 | (argb & 0x00ff_ffff);
        return;
    }
    let dst = fb.pixels[idx];
    let mix = |s: u32, d: u32| -> u32 { (s * a + d * (255 - a)) / 255 };
    let r = mix((argb >> 16) & 0xff, (dst >> 16) & 0xff);
    let g = mix((argb >> 8) & 0xff, (dst >> 8) & 0xff);
    let b = mix(argb & 0xff, dst & 0xff);
    fb.pixels[idx] = 0xff00_0000 | (r << 16) | (g << 8) | b;
}
