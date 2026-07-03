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

use crate::browser::image::store::Decoded;

use super::raster::{Raster, SS};

// Average each supersample block down to the output raster.
pub(super) fn downsample(r: Raster, out_w: u32, out_h: u32) -> Decoded {
    let mut px = vec![0u32; (out_w * out_h) as usize];
    for y in 0..out_h {
        for x in 0..out_w {
            let mut acc = [0u32; 4];
            for dy in 0..SS {
                for dx in 0..SS {
                    let p = r.px[((y * SS + dy) * r.w + x * SS + dx) as usize];
                    acc[0] += p >> 24;
                    acc[1] += (p >> 16) & 0xFF;
                    acc[2] += (p >> 8) & 0xFF;
                    acc[3] += p & 0xFF;
                }
            }
            let n = SS * SS;
            px[(y * out_w + x) as usize] =
                (acc[0] / n) << 24 | (acc[1] / n) << 16 | (acc[2] / n) << 8 | (acc[3] / n);
        }
    }
    Decoded { w: out_w, h: out_h, px }
}
