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

use super::predict_fns::{average2, clamp_add_sub_full, clamp_add_sub_half, select};

// Add two pixels channel by channel, wrapping each byte, the inverse of the
// residual the predictor transform stored.
fn add(a: u32, b: u32) -> u32 {
    let mut o = 0u32;
    for s in [0, 8, 16, 24] {
        let v = (((a >> s) & 0xff) + ((b >> s) & 0xff)) & 0xff;
        o |= v << s;
    }
    o
}

// The predicted pixel for mode m from the left, top, top-right and top-left
// neighbors. Modes follow the VP8L predictor table.
pub(super) fn predict(m: u32, l: u32, t: u32, tr: u32, tl: u32) -> u32 {
    match m {
        0 => 0xff00_0000,
        1 => l,
        2 => t,
        3 => tr,
        4 => tl,
        5 => average2(average2(l, tr), t),
        6 => average2(l, tl),
        7 => average2(l, t),
        8 => average2(tl, t),
        9 => average2(t, tr),
        10 => average2(average2(l, tl), average2(t, tr)),
        11 => select(l, t, tl),
        12 => clamp_add_sub_full(l, t, tl),
        _ => clamp_add_sub_half(average2(l, t), tl),
    }
}

// Undo the predictor transform in place. Each block picks a mode from the
// data image; the first pixel, top row and left column use fixed predictors.
pub(super) fn apply(px: &mut [u32], w: usize, h: usize, data: &[u32], bits: u32, dw: usize) {
    for y in 0..h {
        for x in 0..w {
            let i = y * w + x;
            let pred = if x == 0 && y == 0 {
                0xff00_0000
            } else if y == 0 {
                px[i - 1]
            } else if x == 0 {
                px[i - w]
            } else {
                let m = (data[(y >> bits) * dw + (x >> bits)] >> 8) & 0xff;
                let tr = if x + 1 < w { px[i - w + 1] } else { px[i - w] };
                predict(m, px[i - 1], px[i - w], tr, px[i - w - 1])
            };
            px[i] = add(px[i], pred);
        }
    }
}
