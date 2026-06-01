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

//! Push descriptor 0 into the available ring. virtio-rng has one
//! virtqueue and one descriptor at a time is sufficient: the
//! device fills the buffer descriptor 0 points at and posts the
//! used ring with the byte count.

use core::ptr::{read_volatile, write_volatile};

use super::layout::Queue;
use crate::constants::{VQ_DESC_OFFSET, VRING_DESC_F_WRITE};

const DESC_SIZE: usize = 16; // VirtqDesc { addr, len, flags, next }

impl Queue {
    /// Write descriptor 0 to point at the entropy buffer with the
    /// device-write flag set, then publish slot 0 in the available
    /// ring.
    ///
    /// SAFETY: the descriptor and ring memory belong to the DMA
    /// grant the broker handed the capsule; only the capsule
    /// writes to them between requests, and the device reads them
    /// after the queue-notify register is poked (which the caller
    /// of this function does next).
    pub fn post_request(&self) {
        unsafe {
            let desc_base = self.region_va.add(VQ_DESC_OFFSET);
            // VirtqDesc layout: u64 addr, u32 len, u16 flags, u16 next
            write_volatile(desc_base.cast::<u64>(), self.buf_phys);
            write_volatile(desc_base.add(8).cast::<u32>(), self.buf_len);
            write_volatile(desc_base.add(12).cast::<u16>(), VRING_DESC_F_WRITE);
            write_volatile(desc_base.add(14).cast::<u16>(), 0u16);
            let _ = DESC_SIZE; // documented size; the offsets above derive from it.

            let avail = self.region_va.add(self.avail_offset).cast::<u16>();
            // VirtqAvail layout: u16 flags, u16 idx, u16 ring[QUEUE_SIZE], u16 used_event
            // ring slot 0 lives at offset 4 = avail.add(2)
            write_volatile(avail.add(2), 0u16);
            // Publish: bump idx after the slot write so the device
            // never observes a partial ring update.
            let idx = read_volatile(avail.add(1));
            write_volatile(avail.add(1), idx.wrapping_add(1));
        }
    }
}
