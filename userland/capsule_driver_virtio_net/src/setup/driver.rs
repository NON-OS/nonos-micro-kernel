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

use nonos_libc::{mk_device_release, mk_dma_unmap, mk_irq_unbind};

use super::registers::RegisterGrant;
use crate::constants::MAC_LEN;
use crate::queue::{RxQueue, TxQueue};
use crate::regs::Regs;

pub struct Driver {
    pub device_id: u64,
    pub register: RegisterGrant,
    pub irq_grant: u64,
    pub rx_queue_grant: u64,
    pub rx_buffer_grant: u64,
    pub tx_queue_grant: u64,
    pub tx_buffer_grant: u64,
    pub rx: RxQueue,
    pub tx: TxQueue,
    pub regs: Regs,
    pub mac: [u8; MAC_LEN],
    pub status_supported: bool,
    pub net_status_offset: usize,
}

impl Driver {
    pub fn release(&self) -> bool {
        let tx_buffer = mk_dma_unmap(self.tx_buffer_grant);
        let tx_queue = mk_dma_unmap(self.tx_queue_grant);
        let rx_buffer = mk_dma_unmap(self.rx_buffer_grant);
        let rx_queue = mk_dma_unmap(self.rx_queue_grant);
        let irq = mk_irq_unbind(self.irq_grant);
        let register = self.register.release();
        let device = mk_device_release(self.device_id);
        tx_buffer >= 0
            && tx_queue >= 0
            && rx_buffer >= 0
            && rx_queue >= 0
            && irq >= 0
            && register
            && device >= 0
    }
}
