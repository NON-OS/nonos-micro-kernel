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

// Bits per index and the block-width shift for a palette of this size. Small
// palettes bundle several indices into one pixel's green channel.
pub(super) fn bundle_params(num_colors: usize) -> (u32, u32) {
    match num_colors {
        0..=2 => (1, 3),
        3..=4 => (2, 2),
        5..=16 => (4, 1),
        _ => (8, 0),
    }
}

// Prefix-sum the palette so each stored color is a delta from the previous,
// wrapping each channel.
pub(super) fn delta_decode(pal: &mut [u32]) {
    for i in 1..pal.len() {
        let (a, b) = (pal[i], pal[i - 1]);
        let mut o = 0u32;
        for s in [0, 8, 16, 24] {
            o |= ((((a >> s) & 0xff) + ((b >> s) & 0xff)) & 0xff) << s;
        }
        pal[i] = o;
    }
}

// Expand a bundled index image to full width and map each index to its
// palette color. An out-of-range index paints transparent.
pub(super) fn apply(
    px: &[u32],
    bundled_w: usize,
    h: usize,
    full_w: usize,
    pal: &[u32],
) -> Vec<u32> {
    let (bpp, wbits) = bundle_params(pal.len());
    let per = 1usize << wbits;
    let mask = (1u32 << bpp) - 1;
    let mut out = vec![0u32; full_w * h];
    for y in 0..h {
        for x in 0..full_w {
            let src = px[y * bundled_w + (x >> wbits)];
            let green = (src >> 8) & 0xff;
            let idx = ((green >> ((x % per) as u32 * bpp)) & mask) as usize;
            out[y * full_w + x] = pal.get(idx).copied().unwrap_or(0);
        }
    }
    out
}
