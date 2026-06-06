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

use super::dma_engine::DmaEngine;
use super::dma_types::DmaBuffer;
use super::error::{PciError, PciResult};

impl DmaEngine {
    pub fn alloc_coherent(&mut self, size: usize) -> PciResult<&DmaBuffer> {
        let phys_addr = crate::memory::dma::allocate_dma_buffer(size)
            .map_err(|_| PciError::DmaAllocationFailed { size })?;
        let Some(virt_addr) = crate::memory::phys_to_virt(phys_addr) else {
            let _ = crate::memory::dma::free_dma_buffer(phys_addr, size);
            return Err(PciError::DmaAllocationFailed { size });
        };
        let buffer = DmaBuffer { virt_addr, phys_addr, size, coherent: true };
        self.coherent_buffers.push(buffer);
        self.coherent_buffers.last().ok_or(PciError::DmaAllocationFailed { size })
    }

    pub fn alloc_streaming(&mut self, size: usize) -> PciResult<&DmaBuffer> {
        let phys_addr = crate::memory::dma::allocate_dma_buffer(size)
            .map_err(|_| PciError::DmaAllocationFailed { size })?;
        let Some(virt_addr) = crate::memory::phys_to_virt(phys_addr) else {
            let _ = crate::memory::dma::free_dma_buffer(phys_addr, size);
            return Err(PciError::DmaAllocationFailed { size });
        };
        let buffer = DmaBuffer { virt_addr, phys_addr, size, coherent: false };
        self.streaming_buffers.push(buffer);
        self.streaming_buffers.last().ok_or(PciError::DmaAllocationFailed { size })
    }

    pub fn free_all(&mut self) {
        for buffer in self.coherent_buffers.drain(..) {
            let _ = crate::memory::dma::free_dma_buffer(buffer.phys_addr, buffer.size);
        }
        for buffer in self.streaming_buffers.drain(..) {
            let _ = crate::memory::dma::free_dma_buffer(buffer.phys_addr, buffer.size);
        }
    }
}
