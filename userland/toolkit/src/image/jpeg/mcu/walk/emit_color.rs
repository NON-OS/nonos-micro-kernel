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
use crate::image::jpeg::ycbcr::ycbcr_to_argb8888;

pub fn emit_color(
    out: &mut [u32],
    size: (usize, usize),
    sampling: (usize, usize),
    y_blocks: &[[u8; 64]],
    chroma: (&[u8; 64], &[u8; 64]),
    mcu: (usize, usize),
) {
    let (width, height) = size;
    let (h_max, v_max) = sampling;
    let (mcu_x, mcu_y) = mcu;
    let mcu_w = h_max * 8;
    let mcu_h = v_max * 8;
    let base_x = mcu_x * mcu_w;
    let base_y = mcu_y * mcu_h;
    let mut yy = 0usize;
    while yy < mcu_h {
        let py = base_y + yy;
        if py >= height {
            break;
        }
        let mut xx = 0usize;
        while xx < mcu_w {
            let px = base_x + xx;
            if px < width {
                let yi = (yy / 8) * h_max + (xx / 8);
                let y_val = y_blocks[yi][(yy % 8) * 8 + (xx % 8)];
                let cb_val = chroma.0[(yy / v_max) * 8 + (xx / h_max)];
                let cr_val = chroma.1[(yy / v_max) * 8 + (xx / h_max)];
                out[py * width + px] = ycbcr_to_argb8888(y_val, cb_val, cr_val);
            }
            xx += 1;
        }
        yy += 1;
    }
}
