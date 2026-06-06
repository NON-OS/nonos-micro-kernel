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

use super::decode_color_mcu::decode_color_mcu;
use super::decode_gray_mcu::decode_gray_mcu;
use super::emit_color::emit_color;
use super::emit_grayscale::emit_grayscale;
use super::types::{McuScratch, ScanContext};

pub fn process_mcu(
    ctx: &ScanContext,
    br: &mut BitReader,
    scratch: &mut McuScratch,
    out: &mut [u32],
    size: (usize, usize),
    sampling: (usize, usize),
    mcu: (usize, usize),
) -> Result<(), DecodeError> {
    if ctx.frame.num_comps == 3 {
        let count = decode_color_mcu(
            ctx,
            br,
            &mut scratch.pred,
            &mut scratch.y_blocks,
            &mut scratch.cb_blk,
            &mut scratch.cr_blk,
        )?;
        emit_color(
            out,
            size,
            sampling,
            &scratch.y_blocks[..count],
            (&scratch.cb_blk, &scratch.cr_blk),
            mcu,
        );
    } else {
        decode_gray_mcu(ctx, br, &mut scratch.pred, &mut scratch.y_blocks[0])?;
        emit_grayscale(out, size.0, size.1, &scratch.y_blocks[0], mcu.0, mcu.1);
    }
    Ok(())
}
