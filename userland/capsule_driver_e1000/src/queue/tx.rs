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

//! TX ring state. `post` programs the next descriptor with
//! `EOP|IFCS|RS` and bumps the tail; `done(idx)` polls the
//! per-slot DD bit so the server loop knows the descriptor and
//! its buffer can be reused.

use crate::constants::queue::{
    TX_BUFFER_LEN, TX_CMD_EOP, TX_CMD_IFCS, TX_CMD_RS, TX_DESC_COUNT, TX_STATUS_DD,
};

use super::layout::TxDesc;
use core::ptr::{addr_of, addr_of_mut, read_volatile, write_volatile};

pub struct TxRing {
    pub ring_user_va: u64,
    pub buffer_user_va: u64,
    pub buffer_device_addr: u64,
    pub tail: u16,
}

/*
 * SAFETY anchor for every unsafe in this file: `ring_user_va` is the broker
 * DMA grant taken in `setup::dma`, which covers TX_DESC_COUNT contiguous
 * 16-byte `TxDesc`s, and `tail` is held below TX_DESC_COUNT by construction
 * in `post`.
 */
impl TxRing {
    pub fn new(ring_user_va: u64, buffer_user_va: u64, buffer_device_addr: u64) -> Self {
        Self { ring_user_va, buffer_user_va, buffer_device_addr, tail: 0 }
    }

    /// # Safety
    ///
    /// `idx` is below the descriptor count; the ring memory is the DMA grant
    /// taken in setup, `TX_DESC_COUNT` descriptors long.
    pub unsafe fn descriptor(&self, idx: u16) -> *mut TxDesc {
        (self.ring_user_va as *mut TxDesc).add(idx as usize)
    }

    pub fn buffer_phys(&self, idx: u16) -> u64 {
        self.buffer_device_addr + (idx as u64) * (TX_BUFFER_LEN as u64)
    }

    pub fn buffer_va(&self, idx: u16) -> u64 {
        self.buffer_user_va + (idx as u64) * (TX_BUFFER_LEN as u64)
    }

    pub fn post(&mut self, len: u16) -> u16 {
        let idx = self.tail;
        let desc = unsafe { self.descriptor(idx) };
        /*
         * The part reads the descriptor by DMA once the tail moves, so the
         * fields are stored in program order and none is left in a register.
         */
        unsafe {
            write_volatile(addr_of_mut!((*desc).buffer_addr), self.buffer_phys(idx));
            write_volatile(addr_of_mut!((*desc).length), len);
            write_volatile(addr_of_mut!((*desc).cmd), TX_CMD_EOP | TX_CMD_IFCS | TX_CMD_RS);
            write_volatile(addr_of_mut!((*desc).status), 0);
        }
        self.tail = (self.tail + 1) % (TX_DESC_COUNT as u16);
        idx
    }

    pub fn done(&self, idx: u16) -> bool {
        unsafe { read_volatile(addr_of!((*self.descriptor(idx)).status)) & TX_STATUS_DD != 0 }
    }
}
