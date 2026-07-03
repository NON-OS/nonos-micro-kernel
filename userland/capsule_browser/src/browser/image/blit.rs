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
// ratio and centering. Nearest-neighbor sampling; alpha composited over the
// existing surface so transparent PNGs read correctly.
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
        let src_y = (dy as u64 * img.h as u64 / dst_h as u64) as usize;
        let row = src_y * img.w as usize;
        for dx in 0..dst_w {
            let src_x = (dx as u64 * img.w as u64 / dst_w as u64) as usize;
            put(fb, ox + dx, oy + dy, img.px[row + src_x]);
        }
    }
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
