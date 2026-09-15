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

//! Read the used ring after the device has filled descriptor 0.
//! The byte count comes from the used-elem the device wrote; the
//! capsule then reads up to that many bytes through the buffer
//! mapping.

use core::ptr::read_volatile;

use super::layout::Queue;
use crate::constants::VQ_USED_OFFSET;

// VirtqUsed: u16 flags, u16 idx, VirtqUsedElem ring[QUEUE_SIZE]
// VirtqUsedElem: u32 id, u32 len   (8 bytes; first elem at +4)
const USED_IDX_OFFSET: usize = 2;
const USED_RING_OFFSET: usize = 4;
const USED_ELEM_LEN_OFFSET: usize = 4;

impl Queue {
    /// Snapshot of the device's used-ring `idx` field. The capsule
    /// compares this to its `last_used` to detect completion.
    pub fn used_idx(&self) -> u16 {
        unsafe { read_volatile(self.region_va.add(VQ_USED_OFFSET + USED_IDX_OFFSET).cast()) }
    }

    /// Bytes the device wrote into descriptor 0 for the most
    /// recent completion. Reads the `len` field of used-elem 0.
    pub fn used_len(&self) -> u32 {
        let off = VQ_USED_OFFSET + USED_RING_OFFSET + USED_ELEM_LEN_OFFSET;
        unsafe { read_volatile(self.region_va.add(off).cast()) }
    }

    /// Borrow the entropy buffer the device wrote into. The slice
    /// is capped at `buf_len` so a misbehaving device cannot
    /// induce an out-of-bounds read.
    ///
    /// # Safety
    /// `len` must be the value returned by `used_len` for the
    /// most recent completed descriptor; the caller is responsible
    /// for not aliasing the buffer with concurrent device writes.
    /// # Safety
    ///
    /// The address the caller mapped, at an offset inside it.
    pub unsafe fn buffer(&self, len: u32) -> &[u8] {
        let n = core::cmp::min(len, self.buf_len) as usize;
        core::slice::from_raw_parts(self.buf_va, n)
    }
}
