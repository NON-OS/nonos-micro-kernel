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
use crate::image::jpeg::ycbcr::gray_to_argb8888;

pub fn emit_grayscale(
    out: &mut [u32],
    width: usize,
    height: usize,
    y_blk: &[u8; 64],
    mcu_x: usize,
    mcu_y: usize,
) {
    let base_x = mcu_x * 8;
    let base_y = mcu_y * 8;
    let mut yy = 0usize;
    while yy < 8 {
        let py = base_y + yy;
        if py >= height {
            break;
        }
        let mut xx = 0usize;
        while xx < 8 {
            let px = base_x + xx;
            if px < width {
                let v = y_blk[yy * 8 + xx];
                out[py * width + px] = gray_to_argb8888(v);
            }
            xx += 1;
        }
        yy += 1;
    }
}
