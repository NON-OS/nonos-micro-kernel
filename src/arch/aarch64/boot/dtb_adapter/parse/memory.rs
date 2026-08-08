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
use crate::arch::fdt::find::memory::{find, MemoryRange};
use crate::arch::fdt::Fdt;

pub fn populate(fdt: &Fdt, info: &mut BootInfo) -> bool {
    let mut ranges = [MemoryRange { base: 0, size: 0 }; 8];
    let mem_count = match find(fdt, &mut ranges) {
        Ok(n) => n,
        Err(_) => return false,
    };
    if mem_count == 0 {
        return false;
    }
    info.ram_base = ranges[0].base;
    info.ram_size = ranges[0].size;
    info.memory_region_count = 0;
    for range in ranges.iter().take(mem_count) {
        info.add_memory_region(range.base, range.size, MemoryType::Available);
    }
    true
}
