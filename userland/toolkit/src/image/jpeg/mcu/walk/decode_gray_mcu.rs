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
use crate::image::jpeg::mcu::decode::decode_block;
use crate::image::types::DecodeError;

use super::types::ScanContext;

pub fn decode_gray_mcu(
    ctx: &ScanContext,
    br: &mut BitReader,
    pred: &mut [i32; 3],
    y_blk: &mut [u8; 64],
) -> Result<(), DecodeError> {
    let comp_index = ctx.scan.comps[0].frame_index;
    decode_block(
        br,
        &ctx.dc_tables[ctx.scan.comps[0].td as usize],
        &ctx.ac_tables[ctx.scan.comps[0].ta as usize],
        &ctx.qt[ctx.frame.comps[comp_index].tq as usize],
        &mut pred[0],
        y_blk,
    )
}
