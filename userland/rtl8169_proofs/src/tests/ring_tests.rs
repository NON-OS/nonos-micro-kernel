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

//! Descriptor rings and the registers that point the part at them.

use super::memory::{Memory, RX_BUF_DA, RX_DESC_DA, TX_BUF_DA, TX_DESC_DA};
use super::model::window;
use crate::constants::queue::{BUFFER_SIZE, DESC_BYTES, RX_DESC_COUNT, TX_DESC_COUNT};
use crate::constants::regs::{
    DESC_EOR, DESC_LEN_MASK, DESC_OWN, REG_RMS, REG_RXDESC_ADDR_HI, REG_RXDESC_ADDR_LO,
    REG_TXDESC_ADDR_HI, REG_TXDESC_ADDR_LO,
};
use crate::init::{rx_program, tx_program};
use crate::queue::desc::{desc, Descriptor};

#[test]
fn a_descriptor_is_sixteen_bytes_as_the_part_reads_them() {
    assert_eq!(core::mem::size_of::<Descriptor>(), DESC_BYTES);
}

#[test]
fn receive_descriptors_are_owned_by_the_part_with_the_ring_closed_at_the_end() {
    let bar = window();
    let mut mem = Memory::new();
    let d = mem.driver(&bar);
    rx_program(&d.regs, &d.rx);
    for i in 0..RX_DESC_COUNT {
        let e = unsafe { desc(d.rx.desc_va, i) };
        assert_ne!(e.opts1 & DESC_OWN, 0, "descriptor {i} belongs to the part");
        assert_eq!(e.opts1 & DESC_EOR != 0, i == RX_DESC_COUNT - 1, "EOR only on the last");
        assert_eq!(e.opts1 & DESC_LEN_MASK, BUFFER_SIZE as u32);
        let addr = e.addr_lo as u64 | (e.addr_hi as u64) << 32;
        assert_eq!(addr, RX_BUF_DA + (i * BUFFER_SIZE) as u64, "buffer {i} address");
    }
    assert_eq!(bar.wrote16(REG_RMS) as usize, BUFFER_SIZE);
    assert_eq!(bar.wrote32(REG_RXDESC_ADDR_LO), RX_DESC_DA as u32);
    assert_eq!(bar.wrote32(REG_RXDESC_ADDR_HI), (RX_DESC_DA >> 32) as u32);
}

#[test]
fn transmit_descriptors_stay_with_the_driver_until_a_frame_is_queued() {
    let bar = window();
    let mut mem = Memory::new();
    let d = mem.driver(&bar);
    tx_program(&d.regs, &d.tx);
    for i in 0..TX_DESC_COUNT {
        let e = unsafe { desc(d.tx.desc_va, i) };
        assert_eq!(e.opts1 & DESC_OWN, 0, "descriptor {i} is not handed over empty");
        assert_eq!(e.opts1 & DESC_EOR != 0, i == TX_DESC_COUNT - 1);
        let addr = e.addr_lo as u64 | (e.addr_hi as u64) << 32;
        assert_eq!(addr, TX_BUF_DA + (i * BUFFER_SIZE) as u64);
    }
    assert_eq!(bar.wrote32(REG_TXDESC_ADDR_LO), TX_DESC_DA as u32);
    assert_eq!(bar.wrote32(REG_TXDESC_ADDR_HI), (TX_DESC_DA >> 32) as u32);
}
