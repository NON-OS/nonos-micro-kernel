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

use crate::boot::handoff::BootHandoffV1;
use crate::memory::addr::PhysAddr;
use crate::sys::serial;

pub(crate) fn init_memory(handoff: &BootHandoffV1) {
    let (mut mem_start, mut mem_end) = (0u64, 0u64);
    unsafe {
        for (start, end) in handoff.mmap.usable_regions() {
            if end > start && (end - start) > (mem_end - mem_start) {
                mem_start = start;
                mem_end = end;
            }
        }
    }
    if mem_end <= mem_start || mem_end - mem_start < 0x100000 {
        mem_start = 0x100000;
        mem_end = 0x8000_0000;
    }
    if mem_start < 0x100000 {
        mem_start = 0x100000;
    }
    // On a machine with more than 4GB of RAM the largest usable region (chosen
    // above for the general allocator) is above 4GB, so it holds no memory a
    // 32-bit device DMA descriptor can address. Find a below-4GB usable region
    // that does not overlap the general allocator's region and carve the low DMA
    // pool from its start; that pool is otherwise unmanaged, so the two allocators
    // never hand out the same frame.
    let (dma_base, dma_pages) = find_low_dma_region(handoff, mem_start, mem_end);

    let start = PhysAddr::new(mem_start);
    let end = PhysAddr::new(mem_end);
    match crate::memory::phys::init(start, end) {
        Ok(()) => serial::println(b"[MEM] phys init OK"),
        Err(_) => {
            serial::println(b"[MEM] phys init failed, using fallback");
            init_fallback();
        }
    }
    if !crate::memory::phys::is_initialized() {
        serial::println(b"[MEM] CRITICAL: phys not initialized");
        init_fallback();
    }
    if crate::memory::phys::is_initialized() {
        crate::hardware::broker::dma::init_display_pool();
        crate::hardware::broker::dma::init_low32_pool(dma_base, dma_pages);
    }
}

/// The top a 32-bit DMA address can name.
const DMA_CEILING_32BIT: u64 = 0x1_0000_0000;
/// Skip the lowest memory (legacy/real-mode/SMP-trampoline area) when siting the
/// pool, so it never collides with fixed low-memory uses.
const DMA_POOL_MIN_BASE: u64 = 0x0100_0000; // 16MB

// Pick a below-4GB usable region disjoint from `[main_start, main_end)` and return
// the page-aligned base and page count for the DMA pool, or `(0, 0)` when the
// general allocator already covers low memory (small machines) or none is found.
fn find_low_dma_region(handoff: &BootHandoffV1, main_start: u64, main_end: u64) -> (u64, usize) {
    let want_pages = crate::hardware::broker::dma::low32_capacity_pages();
    let want_bytes = (want_pages as u64) * 0x1000;
    unsafe {
        for (start, end) in handoff.mmap.usable_regions() {
            if end <= start {
                continue;
            }
            let base = ((start.max(DMA_POOL_MIN_BASE)) + 0xFFF) & !0xFFF;
            let top = end.min(DMA_CEILING_32BIT);
            if base >= top || top - base < want_bytes {
                continue;
            }
            // Must not overlap the general allocator's region.
            if base + want_bytes <= main_start || base >= main_end {
                return (base, want_pages);
            }
        }
    }
    (0, 0)
}

fn init_fallback() {
    let regions = [
        (0x100000u64, 0x8000_0000u64),
        (0x100000u64, 0x4000_0000u64),
        (0x200000u64, 0x1000_0000u64),
    ];
    for (start, end) in regions {
        if crate::memory::phys::init(PhysAddr::new(start), PhysAddr::new(end)).is_ok() {
            crate::sys::serial::println(b"[MEM] fallback OK");
            return;
        }
    }
}
