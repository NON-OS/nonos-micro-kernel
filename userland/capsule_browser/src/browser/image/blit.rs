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

use crate::browser::css::ObjectFit;

use super::store::Decoded;

// Scale-blit `img` into the box at (x, y, box_w, box_h) honouring object-fit:
// contain letterboxes the whole image, cover fills the box and crops the
// overflow, fill stretches to the box. Bilinear sampling keeps scaled images
// smooth; alpha is composited so transparent PNGs read correctly.
pub fn blit_into(
    fb: &mut PaintBuffer,
    img: &Decoded,
    dest: [u32; 4],
    fit: ObjectFit,
    alpha: u8,
    clip: Option<[i32; 4]>,
) {
    let [x, y, box_w, box_h] = dest;
    if img.w == 0 || img.h == 0 || box_w == 0 || box_h == 0 || alpha == 0 {
        return;
    }
    let (iw, ih) = (img.w as u64, img.h as u64);
    let (bw, bh) = (box_w as u64, box_h as u64);
    let whole = [0, 0, img.w, img.h];
    match fit {
        // Whole image into the whole box, ignoring aspect ratio.
        ObjectFit::Fill => draw(fb, img, [x, y, box_w, box_h], whole, alpha, clip),
        // Whole image into a centred rect that fits inside the box.
        ObjectFit::Contain => {
            let (dw, dh) = if bw * ih <= bh * iw {
                (box_w, ((bw * ih) / iw) as u32)
            } else {
                (((bh * iw) / ih) as u32, box_h)
            };
            if dw == 0 || dh == 0 {
                return;
            }
            let dest = [x + (box_w - dw) / 2, y + (box_h - dh) / 2, dw, dh];
            draw(fb, img, dest, whole, alpha, clip);
        }
        // A centred crop of the image, with the box's aspect, into the box.
        ObjectFit::Cover => {
            let src = if bw * ih >= bh * iw {
                let sh = (iw * bh) / bw;
                [0, ((ih - sh) / 2) as u32, img.w, (sh as u32).max(1)]
            } else {
                let sw = (ih * bw) / bh;
                [((iw - sw) / 2) as u32, 0, (sw as u32).max(1), img.h]
            };
            draw(fb, img, [x, y, box_w, box_h], src, alpha, clip);
        }
    }
}

// Bilinear-blit the source rect [sx0, sy0, sw, sh] of `img` into the
// destination rect [dx, dy, dw, dh], skipping pixels outside the clip rect so
// an image stays inside an overflow-hidden or rounded container.
fn draw(
    fb: &mut PaintBuffer,
    img: &Decoded,
    dest: [u32; 4],
    src: [u32; 4],
    alpha: u8,
    clip: Option<[i32; 4]>,
) {
    let [dx, dy, dw, dh] = dest;
    let [sx0, sy0, sw, sh] = src;
    for j in 0..dh {
        let gy = (sy0 as u64 * 256 + j as u64 * sh as u64 * 256 / dh as u64) as u32;
        for i in 0..dw {
            let (px, py) = (dx + i, dy + j);
            if let Some([cx0, cy0, cx1, cy1]) = clip {
                if (px as i32) < cx0 || px as i32 >= cx1 || (py as i32) < cy0 || py as i32 >= cy1 {
                    continue;
                }
            }
            let gx = (sx0 as u64 * 256 + i as u64 * sw as u64 * 256 / dw as u64) as u32;
            put(fb, px, py, sample(img, gx, gy), alpha);
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

fn put(fb: &mut PaintBuffer, x: u32, y: u32, argb: u32, alpha: u8) {
    if x >= fb.width || y >= fb.height {
        return;
    }
    let idx = y as usize * fb.stride_words as usize + x as usize;
    if idx >= fb.pixels.len() {
        return;
    }
    // The fragment opacity scales the pixel's own alpha.
    let a = (((argb >> 24) & 0xff) * alpha as u32) / 255;
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
