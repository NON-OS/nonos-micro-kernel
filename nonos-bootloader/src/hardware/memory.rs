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

use uefi::prelude::*;
use uefi::table::boot::{AllocateType, MemoryType};

pub fn discover_memory_size(system_table: &mut SystemTable<Boot>) -> u64 {
    let bs = system_table.boot_services();
    let map_info = bs.memory_map_size();
    let buf_size = map_info.map_size + (map_info.entry_size * 8);
    let pages_needed = buf_size.div_ceil(4096);
    let Ok(ptr) = bs.allocate_pages(AllocateType::AnyPages, MemoryType::LOADER_DATA, pages_needed)
    else {
        return 0;
    };
    let buf = unsafe { core::slice::from_raw_parts_mut(ptr as *mut u8, buf_size) };
    let total =
        bs.memory_map(buf).map(|m| m.entries().map(|d| d.page_count * 4096).sum()).unwrap_or(0);
    let _ = bs.free_pages(ptr, pages_needed);
    total
}
