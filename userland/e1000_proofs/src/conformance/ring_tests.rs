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

//! The rings as the part is told about them, and as the driver primes them.

use super::memory::{Memory, RX_BUF_PHYS, RX_RING_PHYS, TX_RING_PHYS};
use super::model::window;
use crate::constants::queue::{RX_BUFFER_LEN, RX_DESC_COUNT, RX_RING_BYTES, TX_RING_BYTES};
use crate::constants::regs::{
    REG_RCTL, REG_RDBAH, REG_RDBAL, REG_RDH, REG_RDLEN, REG_RDT, REG_TCTL, REG_TDBAH, REG_TDBAL,
    REG_TDH, REG_TDLEN, REG_TDT, REG_TIPG,
};
use crate::init::{rx_program, tx_program};
use crate::regs::Regs;

/// RCTL as the 8254x manual numbers it: EN bit 1, BAM bit 15, SECRC bit 26,
/// BSIZE 00 for 2048-byte buffers.
const RCTL_EXPECTED: u32 = (1 << 1) | (1 << 15) | (1 << 26);
/// TCTL likewise: EN bit 1, PSP bit 3, CT 0x10 at bit 4, COLD 0x40 at bit 12.
const TCTL_EXPECTED: u32 = (1 << 1) | (1 << 3) | (0x10 << 4) | (0x40 << 12);

#[test]
fn the_receive_ring_is_named_primed_with_its_buffers_and_enabled() {
    let bar = window();
    let mut mem = Memory::new();
    let rx = mem.rx_ring();
    rx_program(&Regs::new(bar.base()), &rx, RX_RING_PHYS);
    assert_eq!(bar.wrote32(REG_RDBAL), RX_RING_PHYS as u32);
    assert_eq!(bar.wrote32(REG_RDBAH), (RX_RING_PHYS >> 32) as u32);
    assert_eq!(bar.wrote32(REG_RDLEN), RX_RING_BYTES as u32);
    assert_eq!(bar.wrote32(REG_RDH), 0);
    assert_eq!(bar.wrote32(REG_RDT), RX_DESC_COUNT as u32 - 1, "tail at the last descriptor");
    assert_eq!(bar.wrote32(REG_RCTL), RCTL_EXPECTED);
    for (i, d) in mem.rx.iter().enumerate() {
        assert_eq!(d.buffer_addr, RX_BUF_PHYS + (i * RX_BUFFER_LEN) as u64);
        assert_eq!(d.status, 0, "nothing received yet");
    }
}

#[test]
fn the_transmit_ring_is_named_empty_with_the_standard_gap_and_enabled() {
    let bar = window();
    let mut mem = Memory::new();
    mem.tx[3].cmd = 0xFF;
    let tx = mem.tx_ring();
    tx_program(&Regs::new(bar.base()), &tx, TX_RING_PHYS);
    assert_eq!(bar.wrote32(REG_TDBAL), TX_RING_PHYS as u32);
    assert_eq!(bar.wrote32(REG_TDBAH), (TX_RING_PHYS >> 32) as u32);
    assert_eq!(bar.wrote32(REG_TDLEN), TX_RING_BYTES as u32);
    assert_eq!((bar.wrote32(REG_TDH), bar.wrote32(REG_TDT)), (0, 0));
    assert_eq!(bar.wrote32(REG_TIPG), 0x0060_2008, "IEEE 802.3 full duplex");
    assert_eq!(bar.wrote32(REG_TCTL), TCTL_EXPECTED);
    assert!(mem.tx.iter().all(|d| d.cmd == 0), "the ring was cleared first");
}
