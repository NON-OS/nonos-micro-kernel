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

//! The MPDU RX ring's index bookkeeping. The hardware owns a write index that
//! advances as frames arrive; the host owns a read index that advances as they
//! are consumed and the slot is re-armed. The number of ready frames is the gap
//! from the host read index up to the hardware write index, modulo the ring
//! length, following rtw88 `rtw_pci_get_hw_rx_ring_nr`. Pure arithmetic, checked
//! without hardware.

use super::regs::RX_BUF_DESC_SIZE;

/// Number of descriptors in the RX ring.
pub const RX_DESC_COUNT: u32 = 32;
/// Bytes per RX buffer: the 24-byte descriptor, driver info and a full 802.11
/// frame. 32 slots at this stride is 128 KiB (32 pages), within the 64-page
/// network DMA grant.
pub const RX_BUF_STRIDE: usize = 4096;

/// The host read index into the RX ring.
#[derive(Clone, Copy)]
pub struct RxState {
    pub rp: u32,
    pub len: u32,
    /// Frames the card flagged CRC- or ICV-bad and the poll dropped. Climbing
    /// while a lease is awaited means a reply arrives but does not decrypt (the
    /// group key), so it never reaches the stack.
    pub err_drops: u32,
}

impl RxState {
    pub const fn new(len: u32) -> Self {
        Self { rp: 0, len, err_drops: 0 }
    }

    /// Ready frames: the distance from the host read index up to the hardware
    /// write index, wrapping at the ring end.
    pub fn ready(&self, hw_wp: u32) -> u32 {
        (hw_wp + self.len - self.rp) % self.len
    }

    /// Advance the host read index by one, wrapping at the ring end.
    pub fn advance(&mut self) {
        self.rp += 1;
        if self.rp >= self.len {
            self.rp = 0;
        }
    }

    /// Byte offset of a slot's buffer descriptor in the descriptor ring.
    pub fn desc_offset(slot: u32) -> usize {
        slot as usize * RX_BUF_DESC_SIZE
    }

    /// Byte offset of a slot's frame buffer in the packet buffer.
    pub fn buffer_offset(slot: u32) -> usize {
        slot as usize * RX_BUF_STRIDE
    }
}
