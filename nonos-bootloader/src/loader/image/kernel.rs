// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use crate::crypto::sig::CapsuleMetadata;
use crate::loader::types::memory;

use super::segment_layout::{KernelSegmentLayout, MAX_KERNEL_SEGMENTS};

#[derive(Debug, Clone)]
pub struct KernelImage {
    /// Physical base of the loaded image (where UEFI placed the
    /// PT_LOAD bytes). Pre-paging code dereferences via this.
    pub address: usize,
    /// Total memory span across all PT_LOADs.
    pub size: usize,
    /// Architectural entry point. For upper-half kernels this is a
    /// virt address that only becomes reachable after the bootloader
    /// CR3 swap; for legacy low-half ET_EXEC it equals the phys entry.
    pub entry_point: usize,
    /// Upper-half virt base, or 0 when the image is loaded at its
    /// physical address (legacy low-half ET_EXEC / ET_DYN).
    pub virt_base: u64,
    pub segments: [KernelSegmentLayout; MAX_KERNEL_SEGMENTS],
    pub segment_count: usize,
    pub metadata: CapsuleMetadata,
    pub allocations: [(u64, usize); memory::MAX_ALLOCATIONS],
    pub alloc_count: usize,
}

impl KernelImage {
    pub fn new(address: usize, size: usize, entry_point: usize, metadata: CapsuleMetadata) -> Self {
        Self {
            address,
            size,
            entry_point,
            virt_base: 0,
            segments: [KernelSegmentLayout::default(); MAX_KERNEL_SEGMENTS],
            segment_count: 0,
            metadata,
            allocations: [(0, 0); memory::MAX_ALLOCATIONS],
            alloc_count: 0,
        }
    }

    pub fn with_allocations(
        address: usize,
        size: usize,
        entry_point: usize,
        metadata: CapsuleMetadata,
        allocations: [(u64, usize); memory::MAX_ALLOCATIONS],
        alloc_count: usize,
    ) -> Self {
        Self {
            address,
            size,
            entry_point,
            virt_base: 0,
            segments: [KernelSegmentLayout::default(); MAX_KERNEL_SEGMENTS],
            segment_count: 0,
            metadata,
            allocations,
            alloc_count,
        }
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.address as *const u8
    }

    pub fn as_mut_ptr(&self) -> *mut u8 {
        self.address as *mut u8
    }

    pub unsafe fn as_slice(&self) -> &[u8] {
        core::slice::from_raw_parts(self.as_ptr(), self.size)
    }

    pub fn end_address(&self) -> usize {
        self.address + self.size
    }

    pub fn contains(&self, addr: usize) -> bool {
        addr >= self.address && addr < self.end_address()
    }

    pub fn page_count(&self) -> usize {
        memory::pages_needed(self.size)
    }

    pub fn total_allocated_pages(&self) -> usize {
        self.allocations[..self.alloc_count].iter().map(|(_, pages)| *pages).sum()
    }

    /// Range-check the entry point. Upper-half images compare against
    /// the virt window; legacy images keep the phys check.
    pub fn is_entry_valid(&self) -> bool {
        if self.virt_base != 0 {
            let base = self.virt_base as usize;
            self.entry_point >= base && self.entry_point < base + self.size
        } else {
            self.contains(self.entry_point)
        }
    }

    pub fn is_upper_half(&self) -> bool {
        self.virt_base != 0
    }

    pub fn segments(&self) -> &[KernelSegmentLayout] {
        &self.segments[..self.segment_count]
    }

    pub fn payload_hash(&self) -> &[u8; 32] {
        &self.metadata.payload_hash
    }

    pub fn is_signed(&self) -> bool {
        self.metadata.len_sig > 0
    }

    pub fn signer_key_id(&self) -> Option<[u8; 32]> {
        self.metadata.signer_keyid
    }
}

impl Default for KernelImage {
    fn default() -> Self {
        Self {
            address: 0,
            size: 0,
            entry_point: 0,
            virt_base: 0,
            segments: [KernelSegmentLayout::default(); MAX_KERNEL_SEGMENTS],
            segment_count: 0,
            metadata: CapsuleMetadata::default(),
            allocations: [(0, 0); memory::MAX_ALLOCATIONS],
            alloc_count: 0,
        }
    }
}
