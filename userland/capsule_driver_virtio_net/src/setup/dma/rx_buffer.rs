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

use nonos_libc::{mk_dma_map, DmaMapOut, IrqBindOut};

use super::super::registers::RegisterGrant;
use super::rollback;
use crate::constants::{RX_BUFFER_LEN, RX_DESC_COUNT};

pub fn map_rx_buffers(
    device_id: u64,
    claim_epoch: u64,
    reg: &RegisterGrant,
    irq: &IrqBindOut,
    rx_queue: &DmaMapOut,
) -> Result<DmaMapOut, &'static str> {
    let mut out = DmaMapOut { user_va: 0, device_addr: 0, length: 0, grant_id: 0 };
    let len = (RX_BUFFER_LEN as u64) * (RX_DESC_COUNT as u64);
    let r = mk_dma_map(device_id, claim_epoch, len, 0, &mut out);
    if r >= 0 {
        return Ok(out);
    }
    if !rollback::after(device_id, reg, irq, &[rx_queue.grant_id]) {
        return Err("dma rollback failed (rx buffers)");
    }
    Err("dma map failed (rx buffers)")
}
