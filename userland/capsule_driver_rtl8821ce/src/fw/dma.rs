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

//! The DMA-memory seam the reserved-page staging is written against. Staging a
//! firmware chunk means placing bytes in a host buffer the card reads by bus
//! address, so the code needs both a CPU view (to write) and the device address
//! (to hand the card). A real buffer backs this with a broker `mk_dma_map` grant
//! (`user_va` for the writes, `device_addr` for the card); a host proof backs it
//! with a plain byte vector, so the exact bytes and descriptor the card would
//! see are checked without hardware.

/// A block of DMA-coherent memory: a CPU-writable region with a stable device
/// (bus) address. Offsets are in bytes from the start of the region.
pub trait DmaMem {
    /// Total size of the region in bytes.
    fn capacity(&self) -> usize;
    /// The device (bus) address of the region's start.
    fn device_addr(&self) -> u64;
    /// Copy `src` into the region at `offset`. Callers keep writes in bounds.
    fn write_bytes(&self, offset: usize, src: &[u8]);
}

/// A DMA region backed by a live broker grant: `user_va` is the mapped CPU
/// address, `device_addr` the physical address the card sees. The grant keeps
/// the mapping alive for the driver's lifetime.
pub struct Grant {
    user_va: u64,
    device_addr: u64,
    length: usize,
}

impl Grant {
    pub const fn new(user_va: u64, device_addr: u64, length: usize) -> Self {
        Self { user_va, device_addr, length }
    }

    /// A CPU-readable view of the whole region. The receive path reads completed
    /// frames out of its buffers this way.
    pub fn as_slice(&self) -> &[u8] {
        // SAFETY: the grant maps [user_va, user_va+length) as write-back coherent
        // host memory for the driver's lifetime, so a shared byte view is sound;
        // the card only ever writes frames the driver then reads here.
        unsafe { core::slice::from_raw_parts(self.user_va as *const u8, self.length) }
    }
}

impl DmaMem for Grant {
    fn capacity(&self) -> usize {
        self.length
    }
    fn device_addr(&self) -> u64 {
        self.device_addr
    }
    fn write_bytes(&self, offset: usize, src: &[u8]) {
        // SAFETY: the region is [user_va, user_va+length); callers keep
        // offset+src.len() within it, and the mapping is write-back coherent so
        // a byte copy is what the card later reads. Written volatile so the copy
        // is not reordered past the register kick that follows.
        let base = (self.user_va as usize + offset) as *mut u8;
        for (i, b) in src.iter().enumerate() {
            unsafe { core::ptr::write_volatile(base.add(i), *b) }
        }
    }
}
