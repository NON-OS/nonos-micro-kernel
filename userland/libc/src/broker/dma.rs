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

//! DMA buffer mapping. Cap requirement: `Dma`.

use super::types::DmaMapOut;
use crate::syscall::{call_raw, N_MK_DMA_MAP, N_MK_DMA_UNMAP};

pub const MK_DMA_MAP_HIGH: u32 = 1 << 0;

#[no_mangle]
pub extern "C" fn mk_dma_map(
    device_id: u64,
    claim_epoch: u64,
    length: u64,
    flags: u32,
    out: *mut DmaMapOut,
) -> i64 {
    call_raw(N_MK_DMA_MAP, [device_id, claim_epoch, length, flags as u64, out as u64, 0])
}

#[no_mangle]
pub extern "C" fn mk_dma_unmap(grant_id: u64) -> i64 {
    call_raw(N_MK_DMA_UNMAP, [grant_id, 0, 0, 0, 0, 0])
}
