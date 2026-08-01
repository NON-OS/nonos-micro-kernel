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

//! Handing the device tree's memory map to the physical allocator.
//!
//! The x86_64 path walks EFI descriptors; here the regions came out of the FDT
//! and are already normalised into [`MemoryRegion`]. What both paths owe the
//! allocator is the same: one contiguous managed range, plus a below-4 GiB pool
//! for devices whose DMA descriptors are only 32 bits wide.

use super::info::{BootInfo, MemoryRegion, MemoryType};
use crate::hardware::broker::dma;
use crate::memory::addr::PhysAddr;
use crate::sys::serial;

/// The top a 32-bit DMA address can name.
const DMA_CEILING_32BIT: u64 = 0x1_0000_0000;
/// Keep the pool clear of the bottom of RAM, where the kernel image and the
/// secondary-CPU trampoline live.
const DMA_POOL_MIN_BASE: u64 = 0x0100_0000;
const PAGE_SIZE: u64 = 0x1000;

extern "C" {
    static __kernel_image_end: u8;
}

pub(crate) fn init_boot_memory(info: &BootInfo) {
    let Some((region_start, end)) = largest_available(info) else {
        serial::println(b"[MEM] no usable region in device tree");
        return;
    };

    // The image sits inside the region firmware calls available, so the
    // allocator has to be told where it ends. A frame holding kernel text is
    // not a fault the kernel survives to report: it is handed out as a fresh
    // page table, zeroed, and the next call into whatever lived there executes
    // zeros.
    let image_end = (&raw const __kernel_image_end) as u64;
    let start = if image_end > region_start && image_end < end {
        (image_end + PAGE_SIZE - 1) & !(PAGE_SIZE - 1)
    } else {
        region_start
    };
    if start >= end {
        serial::println(b"[MEM] nothing usable above the kernel image");
        return;
    }

    let (dma_base, dma_pages) = find_low_dma_region(info, start, end);

    match crate::memory::phys::init(PhysAddr::new(start), PhysAddr::new(end)) {
        Ok(()) => serial::println(b"[MEM] phys init OK"),
        Err(_) => {
            serial::println(b"[MEM] phys init failed");
            return;
        }
    }

    dma::init_display_pool();
    dma::init_low32_pool(dma_base, dma_pages);
}

/// The widest `Available` region the device tree reported, page aligned inward.
fn largest_available(info: &BootInfo) -> Option<(u64, u64)> {
    info.memory_map()
        .iter()
        .filter(|r| r.region_type == MemoryType::Available)
        .filter_map(|r| bounds(r))
        .max_by_key(|(start, end)| end - start)
}

/// Page-aligned `[start, end)` for a region, or `None` if nothing is left of
/// it once both ends are aligned inward.
fn bounds(region: &MemoryRegion) -> Option<(u64, u64)> {
    let start = region.base.checked_add(PAGE_SIZE - 1)? & !(PAGE_SIZE - 1);
    let end = region.base.checked_add(region.size)? & !(PAGE_SIZE - 1);
    (end > start).then_some((start, end))
}

/// A below-4 GiB window that does not overlap the general allocator's range,
/// or `(0, 0)` when there is none. Overlapping the two would let both hand out
/// the same frame.
fn find_low_dma_region(info: &BootInfo, main_start: u64, main_end: u64) -> (u64, usize) {
    let want_pages = dma::low32_capacity_pages();
    let want_bytes = (want_pages as u64) * PAGE_SIZE;

    for region in info.memory_map().iter() {
        if region.region_type != MemoryType::Available {
            continue;
        }
        let Some((start, end)) = bounds(region) else {
            continue;
        };
        let base = start.max(DMA_POOL_MIN_BASE);
        let top = end.min(DMA_CEILING_32BIT);
        if base >= top || top - base < want_bytes {
            continue;
        }
        if base + want_bytes <= main_start || base >= main_end {
            return (base, want_pages);
        }
    }
    (0, 0)
}
