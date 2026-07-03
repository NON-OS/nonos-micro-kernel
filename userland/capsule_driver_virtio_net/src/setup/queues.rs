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

use super::dma_set::DmaSet;
use crate::constants::{Q_RX, Q_TX, VQ_REGION_SIZE};
use crate::init::program_queue;
use crate::queue::{clear_region, RxQueue, TxQueue};
use crate::regs::Regs;

pub fn build(regs: Regs, dma: &DmaSet) -> Result<(RxQueue, TxQueue), &'static str> {
    unsafe {
        clear_region(dma.rx_queue.user_va, VQ_REGION_SIZE);
        clear_region(dma.tx_queue.user_va, VQ_REGION_SIZE);
    }
    let rx_count = program_queue(regs, Q_RX, dma.rx_queue.device_addr, RxQueue::queue_size())?;
    let tx_count = program_queue(regs, Q_TX, dma.tx_queue.device_addr, TxQueue::queue_size())?;
    let rx = RxQueue::new(
        dma.rx_queue.user_va,
        dma.rx_queue.device_addr,
        dma.rx_buffer.user_va,
        dma.rx_buffer.device_addr,
        rx_count,
    );
    let tx = TxQueue::new(
        dma.tx_queue.user_va,
        dma.tx_queue.device_addr,
        dma.tx_buffer.user_va,
        dma.tx_buffer.device_addr,
        tx_count,
    );
    Ok((rx, tx))
}
