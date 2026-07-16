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

//! The BE TX ring's index bookkeeping. The host owns a write index that advances
//! as frames are queued; the hardware owns a read index that advances as they
//! are sent. One descriptor is always left free so a full ring is distinguishable
//! from an empty one, following rtw88 `avail_desc` in `pci.h`. This is pure
//! arithmetic over the two indices and the ring length, checked without hardware.

use crate::ring::{TX_BUF_DESC_SIZE, TX_DESC_SIZE};

/// Number of descriptors in the BE ring.
pub const TX_DESC_COUNT: u32 = 64;
/// Bytes reserved per frame in the packet buffer: the 48-byte descriptor plus a
/// full 802.11 MTU, rounded up. 64 slots at this stride is 128 KiB (32 pages),
/// within the 64-page network DMA grant.
pub const TX_BUF_STRIDE: usize = 2048;

// The stride must hold a descriptor plus a frame; a slot never overflows.
const _: () = assert!(TX_BUF_STRIDE > TX_DESC_SIZE as usize);

/// The host and hardware indices into the BE ring.
#[derive(Clone, Copy)]
pub struct TxState {
    pub wp: u32,
    pub rp: u32,
    pub len: u32,
}

impl TxState {
    pub const fn new(len: u32) -> Self {
        Self { wp: 0, rp: 0, len }
    }

    /// Free descriptors, leaving one reserved so full and empty differ.
    pub fn avail(&self) -> u32 {
        if self.rp > self.wp {
            self.rp - self.wp - 1
        } else {
            self.len - self.wp + self.rp - 1
        }
    }

    /// Advance the host write index by one, wrapping at the ring end.
    pub fn advance(&mut self) {
        self.wp += 1;
        if self.wp >= self.len {
            self.wp = 0;
        }
    }

    /// Adopt the hardware read index reported by the index register.
    pub fn set_hw_rp(&mut self, rp: u32) {
        self.rp = rp % self.len;
    }

    /// Byte offset of a slot's buffer descriptor in the descriptor ring.
    pub fn desc_offset(slot: u32) -> usize {
        slot as usize * TX_BUF_DESC_SIZE
    }

    /// Byte offset of a slot's frame buffer in the packet buffer.
    pub fn buffer_offset(slot: u32) -> usize {
        slot as usize * TX_BUF_STRIDE
    }
}
