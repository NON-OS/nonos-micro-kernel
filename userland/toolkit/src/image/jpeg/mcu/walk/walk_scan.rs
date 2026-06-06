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
use crate::image::jpeg::bits::BitReader;
use crate::image::types::DecodeError;

use super::process_mcu::process_mcu;
use super::restart_after_mcu::restart_after_mcu;
use super::types::{McuScratch, ScanContext};

pub fn walk_scan(
    ctx: &ScanContext,
    entropy: &[u8],
    entropy_start: usize,
    out: &mut [u32],
) -> Result<usize, DecodeError> {
    let width = ctx.frame.width as usize;
    let height = ctx.frame.height as usize;
    let h_max = ctx.frame.h_max as usize;
    let v_max = ctx.frame.v_max as usize;
    let mcus_x = (width + h_max * 8 - 1) / (h_max * 8);
    let mcus_y = (height + v_max * 8 - 1) / (v_max * 8);
    let mut br = BitReader::new(entropy, entropy_start);
    let mut scratch =
        McuScratch { pred: [0; 3], y_blocks: [[0; 64]; 4], cb_blk: [0; 64], cr_blk: [0; 64] };
    let mut next_rst: u8 = 0xD0;
    let mut mcus_since_rst: u32 = 0;
    let mut my = 0usize;
    while my < mcus_y {
        let mut mx = 0usize;
        while mx < mcus_x {
            process_mcu(
                ctx,
                &mut br,
                &mut scratch,
                out,
                (width, height),
                (h_max, v_max),
                (mx, my),
            )?;
            restart_after_mcu(
                ctx,
                &mut br,
                &mut scratch.pred,
                &mut next_rst,
                &mut mcus_since_rst,
                (mx, my),
                (mcus_x, mcus_y),
            )?;
            mx += 1;
        }
        my += 1;
    }
    br.align_to_byte();
    Ok(br.pos)
}
