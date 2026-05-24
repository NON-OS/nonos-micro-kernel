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

use crate::arch::aarch64::boot::info::{BootInfo, MemoryType};
use crate::arch::fdt::find::cpus;
use crate::arch::fdt::find::gic::{find as find_gic, GicVersion};
use crate::arch::fdt::find::memory::{find as find_memory, MemoryRange};
use crate::arch::fdt::find::timer::find as find_timer;
use crate::arch::fdt::find::uart::{find as find_uart, UartKind};
use crate::arch::fdt::Fdt;

pub fn populate(dtb_ptr: u64, info: &mut BootInfo) -> bool {
    let fdt = match Fdt::from_ptr(dtb_ptr as *const u8) {
        Ok(f) => f,
        Err(_) => return false,
    };
    info.dtb_base = dtb_ptr;
    info.dtb_size = fdt.header.totalsize as u64;

    let mut ranges = [MemoryRange { base: 0, size: 0 }; 8];
    let mem_count = match find_memory(&fdt, &mut ranges) {
        Ok(n) => n,
        Err(_) => return false,
    };
    if mem_count == 0 {
        return false;
    }
    info.ram_base = ranges[0].base;
    info.ram_size = ranges[0].size;
    info.memory_regions.clear();
    for range in ranges.iter().take(mem_count) {
        info.add_memory_region(range.base, range.size, MemoryType::Available);
    }

    if let Ok(Some(u)) = find_uart(&fdt) {
        if u.kind == UartKind::Pl011 {
            info.uart_base = u.base;
        }
    }

    if let Ok(Some(g)) = find_gic(&fdt) {
        info.gic_dist_base = g.dist_base;
        match g.version {
            GicVersion::V3 => {
                info.gic_redist_base = g.redist_or_cpu_base;
            }
            GicVersion::V2 => {
                info.gic_unsupported = true;
            }
        }
    }

    if let Ok(Some(t)) = find_timer(&fdt) {
        info.timer_phys_intid = t.nonsecure_phys_intid;
        info.timer_virt_intid = t.virtual_intid;
    }

    let mut cpu_ids = [0u64; 64];
    if let Ok(n) = cpus::find(&fdt, &mut cpu_ids) {
        if n > 0 {
            info.cpu_count = n as u32;
        }
    }

    true
}
