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

use nonos_libc::{DmaMapOut, IrqBindOut, MmioMapOut};

use super::dma;

pub struct DmaSet {
    pub rx_queue: DmaMapOut,
    pub rx_buffer: DmaMapOut,
    pub tx_queue: DmaMapOut,
    pub tx_buffer: DmaMapOut,
}

pub fn map(
    device_id: u64,
    claim_epoch: u64,
    mmio: &MmioMapOut,
    irq: &IrqBindOut,
) -> Result<DmaSet, &'static str> {
    let rx_queue = dma::map_rx_queue(device_id, claim_epoch, mmio, irq)?;
    let rx_buffer = dma::map_rx_buffers(device_id, claim_epoch, mmio, irq, &rx_queue)?;
    let tx_queue = dma::map_tx_queue(device_id, claim_epoch, mmio, irq, &rx_queue, &rx_buffer)?;
    let tx_buffer =
        dma::map_tx_buffer(device_id, claim_epoch, mmio, irq, &rx_queue, &rx_buffer, &tx_queue)?;
    Ok(DmaSet { rx_queue, rx_buffer, tx_queue, tx_buffer })
}
