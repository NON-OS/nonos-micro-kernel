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

use super::types::MemoryMapEntry;
use crate::handoff::prepare::MAX_MMAP_ENTRIES;
use crate::handoff::types::{BootHandoffV1, MemoryMap};
use uefi::table::boot::MemoryMap as UefiMemoryMap;

/// Copy UEFI memory map to kernel-readable format. Called after ExitBootServices.
pub fn copy_memory_map(mmap_addr: u64, final_mmap: &UefiMemoryMap<'_>) -> (u64, u32, u32) {
    if mmap_addr == 0 {
        return (0, 0, 0);
    }
    let mmap_buffer = mmap_addr as *mut MemoryMapEntry;
    let mut entry_count: u32 = 0;
    // SAFETY: mmap_addr allocated by prepare_handoff_allocation with sufficient size
    unsafe {
        for desc in final_mmap.entries() {
            if (entry_count as usize) >= MAX_MMAP_ENTRIES {
                break;
            }
            let entry = mmap_buffer.add(entry_count as usize);
            (*entry).memory_type = desc.ty.0;
            (*entry)._pad = 0;
            (*entry).physical_start = desc.phys_start;
            (*entry).virtual_start = desc.virt_start;
            (*entry).page_count = desc.page_count;
            (*entry).attribute = desc.att.bits();
            entry_count += 1;
        }
    }
    (mmap_addr, MemoryMapEntry::SIZE as u32, entry_count)
}

/// Write final memory map info to handoff struct.
pub fn finalize_mmap(
    bh_ptr: *mut BootHandoffV1,
    mmap_addr: u64,
    entry_size: u32,
    entry_count: u32,
) {
    if bh_ptr.is_null() {
        return;
    }
    // SAFETY: bh_ptr validated by caller, points to allocated BootHandoffV1
    unsafe {
        (*bh_ptr).mmap = MemoryMap { ptr: mmap_addr, entry_size, entry_count, desc_version: 1 };
    }
}
