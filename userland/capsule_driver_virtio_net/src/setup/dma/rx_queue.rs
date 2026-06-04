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
use crate::constants::VQ_REGION_SIZE;

pub fn map_rx_queue(
    device_id: u64,
    claim_epoch: u64,
    reg: &RegisterGrant,
    irq: &IrqBindOut,
) -> Result<DmaMapOut, &'static str> {
    let mut out = DmaMapOut { user_va: 0, device_addr: 0, length: 0, grant_id: 0 };
    let r = mk_dma_map(device_id, claim_epoch, VQ_REGION_SIZE as u64, 0, &mut out);
    if r >= 0 {
        return Ok(out);
    }
    if !rollback::after(device_id, reg, irq, &[]) {
        return Err("dma rollback failed (rx queue)");
    }
    Err("dma map failed (rx queue)")
}
