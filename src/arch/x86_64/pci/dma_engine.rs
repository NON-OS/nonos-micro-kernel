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

use super::device::PciDevice;
use super::dma_types::DmaBuffer;
use super::error::{PciError, PciResult};
use super::stats::PCI_STATS;
use alloc::vec::Vec;

pub struct DmaEngine {
    pub(super) device: PciDevice,
    pub(super) coherent_buffers: Vec<DmaBuffer>,
    pub(super) streaming_buffers: Vec<DmaBuffer>,
    pub(super) total_transfers: u64,
    pub(super) total_bytes: u64,
}

impl DmaEngine {
    pub fn new(device: PciDevice) -> PciResult<Self> {
        device.enable_bus_mastering()?;
        device.enable_memory_space()?;
        {
            let mut s = PCI_STATS.write();
            s.dma_engines += 1;
        }
        Ok(Self {
            device,
            coherent_buffers: Vec::new(),
            streaming_buffers: Vec::new(),
            total_transfers: 0,
            total_bytes: 0,
        })
    }
    pub fn device(&self) -> &PciDevice {
        &self.device
    }
    pub fn stats(&self) -> (u64, u64) {
        (self.total_transfers, self.total_bytes)
    }
    fn do_alloc(&mut self, size: usize, coherent: bool) -> PciResult<DmaBuffer> {
        let phys = crate::memory::dma::allocate_dma_buffer(size)
            .map_err(|_| PciError::DmaAllocationFailed { size })?;
        let Some(virt_addr) = crate::memory::phys_to_virt(phys) else {
            let _ = crate::memory::dma::free_dma_buffer(phys, size);
            return Err(PciError::DmaAllocationFailed { size });
        };
        Ok(DmaBuffer {
            virt_addr,
            phys_addr: phys,
            size,
            coherent,
        })
    }
    pub fn alloc_coherent(&mut self, size: usize) -> PciResult<&DmaBuffer> {
        let buf = self.do_alloc(size, true)?;
        self.coherent_buffers.push(buf);
        self.coherent_buffers.last().ok_or(PciError::DmaAllocationFailed { size })
    }
    pub fn alloc_streaming(&mut self, size: usize) -> PciResult<&DmaBuffer> {
        let buf = self.do_alloc(size, false)?;
        self.streaming_buffers.push(buf);
        self.streaming_buffers.last().ok_or(PciError::DmaAllocationFailed { size })
    }
    pub fn free_all(&mut self) {
        for buf in self.coherent_buffers.drain(..).chain(self.streaming_buffers.drain(..)) {
            let _ = crate::memory::dma::free_dma_buffer(buf.phys_addr, buf.size);
        }
    }
}

impl Drop for DmaEngine {
    fn drop(&mut self) {
        self.free_all();
        let mut s = PCI_STATS.write();
        s.dma_engines = s.dma_engines.saturating_sub(1);
    }
}
