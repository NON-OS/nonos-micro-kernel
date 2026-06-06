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

use super::handle_restart::handle_restart;
use super::types::ScanContext;

pub fn restart_after_mcu(
    ctx: &ScanContext,
    br: &mut BitReader,
    pred: &mut [i32; 3],
    next_rst: &mut u8,
    mcus_since_rst: &mut u32,
    mcu: (usize, usize),
    limits: (usize, usize),
) -> Result<(), DecodeError> {
    *mcus_since_rst += 1;
    if ctx.restart_interval == 0 || *mcus_since_rst != ctx.restart_interval {
        return Ok(());
    }
    if mcu.0 + 1 == limits.0 && mcu.1 + 1 == limits.1 {
        return Ok(());
    }
    *next_rst = handle_restart(br, *next_rst, pred)?;
    *mcus_since_rst = 0;
    Ok(())
}
