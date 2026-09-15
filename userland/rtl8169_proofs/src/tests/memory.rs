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

//! Rings and buffers in host memory, and the driver record built over them.

use nonos_devmodel::FakeBar;

use crate::constants::queue::{RX_BUFFER_BYTES, RX_RING_BYTES, TX_BUFFER_BYTES, TX_RING_BYTES};
use crate::constants::MAC_LEN;
use crate::queue::{RxRing, TxRing};
use crate::regs::Regs;
use crate::setup::Driver;

/// Rings and buffers in host memory. Device addresses are made up, since only
/// their encoding into registers and descriptors is under test.
pub struct Memory {
    pub rx_desc: Vec<u8>,
    pub rx_buf: Vec<u8>,
    pub tx_desc: Vec<u8>,
    pub tx_buf: Vec<u8>,
}

pub const RX_DESC_DA: u64 = 0x0000_0001_2000_0000;
pub const RX_BUF_DA: u64 = 0x0000_0001_3000_0000;
pub const TX_DESC_DA: u64 = 0x0000_0002_2000_0000;
pub const TX_BUF_DA: u64 = 0x0000_0002_3000_0000;

impl Memory {
    pub fn new() -> Self {
        Self {
            rx_desc: vec![0; RX_RING_BYTES],
            rx_buf: vec![0; RX_BUFFER_BYTES],
            tx_desc: vec![0; TX_RING_BYTES],
            tx_buf: vec![0; TX_BUFFER_BYTES],
        }
    }
    pub fn driver(&mut self, bar: &FakeBar) -> Driver {
        let va = |v: &mut Vec<u8>| v.as_mut_ptr() as u64;
        Driver {
            device_id: 1,
            mmio_grant: 2,
            irq_grant: 3,
            rx_ring_grant: 4,
            rx_buffer_grant: 5,
            tx_ring_grant: 6,
            tx_buffer_grant: 7,
            regs: Regs::new(bar.base()),
            mac: [0; MAC_LEN],
            rx: RxRing::new(va(&mut self.rx_desc), va(&mut self.rx_buf), RX_DESC_DA, RX_BUF_DA),
            tx: TxRing::new(va(&mut self.tx_desc), va(&mut self.tx_buf), TX_DESC_DA, TX_BUF_DA),
        }
    }
}
