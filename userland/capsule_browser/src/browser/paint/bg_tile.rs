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

use crate::browser::css::{BgSize, ObjectFit};
use crate::browser::image::Decoded;
use crate::browser::layout::boxmodel::Fragment;

// Paint an auto or length background layer: the tile keeps the image aspect at
// its natural or scaled width and repeats across the box per background-repeat,
// which is how CSS lays a pattern by default. `dest` is the box rect and the
// clip keeps every tile inside the fragment.
pub(super) fn paint_tiles(
    fb: &mut PaintBuffer,
    img: &Decoded,
    f: &Fragment,
    dest: [u32; 4],
    clip: Option<[i32; 4]>,
) {
    let [bx, by, bw, bh] = dest;
    if img.w == 0 || img.h == 0 || bw == 0 || bh == 0 {
        return;
    }
    let tw = match f.bg_size {
        BgSize::Px(px) => (px as u32).max(1),
        _ => img.w,
    };
    let th = ((tw as u64 * img.h as u64) / img.w as u64).max(1) as u32;
    if !f.bg_repeat {
        crate::browser::image::blit_into(
            fb,
            img,
            [bx, by, tw.min(bw), th.min(bh)],
            ObjectFit::Fill,
            f.alpha,
            clip,
        );
        return;
    }
    // Bound the tile count so a one-pixel pattern cannot spin the painter.
    let cols = (bw + tw - 1) / tw;
    let rows = (bh + th - 1) / th;
    if cols.saturating_mul(rows) > 4096 {
        crate::browser::image::blit_into(
            fb,
            img,
            [bx, by, bw, bh],
            ObjectFit::Cover,
            f.alpha,
            clip,
        );
        return;
    }
    for r in 0..rows {
        for c in 0..cols {
            crate::browser::image::blit_into(
                fb,
                img,
                [bx + c * tw, by + r * th, tw, th],
                ObjectFit::Fill,
                f.alpha,
                clip,
            );
        }
    }
}
