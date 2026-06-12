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

use super::cmdline::allocate_cmdline;
use super::constants::MMAP_PAGES;
use super::fatal::fatal_alloc_error;
use uefi::prelude::*;
use uefi::table::boot::{AllocateType, MemoryType};

/// Memory allocations for kernel handoff.
pub struct HandoffAllocations {
    pub boothandoff_addr: u64,
    pub stack_addr: u64,
    pub stack_top: usize,
    pub mmap_addr: u64,
    pub cmdline_addr: u64,
}

/// Allocate all memory needed for handoff: struct, stack (64KB), mmap buffer, cmdline.
pub fn allocate_handoff_resources(
    st: &SystemTable<Boot>,
    cmdline: Option<&str>,
) -> HandoffAllocations {
    let bs = st.boot_services();
    let bh_addr = bs
        .allocate_pages(AllocateType::AnyPages, MemoryType::LOADER_DATA, 1)
        .unwrap_or_else(|_| fatal_alloc_error(st, "BootHandoff"));
    let stack_pages: usize = 16;
    let stack_addr = bs
        .allocate_pages(AllocateType::AnyPages, MemoryType::LOADER_DATA, stack_pages)
        .unwrap_or_else(|_| fatal_alloc_error(st, "stack"));
    let stack_top = (stack_addr as usize) + (stack_pages * 0x1000) - 16;
    let mmap_addr = bs
        .allocate_pages(AllocateType::AnyPages, MemoryType::LOADER_DATA, MMAP_PAGES)
        .unwrap_or_else(|_| fatal_alloc_error(st, "mmap"));
    let cmdline_addr = allocate_cmdline(bs, cmdline);
    HandoffAllocations { boothandoff_addr: bh_addr, stack_addr, stack_top, mmap_addr, cmdline_addr }
}
