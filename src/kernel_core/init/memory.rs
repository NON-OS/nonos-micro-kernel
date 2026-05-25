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
    }
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
