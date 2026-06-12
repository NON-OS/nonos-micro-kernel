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

use super::{
    constants::MIN_TEST_ADDR, display::print_results, test::test_region, types::MemTestResult,
};
use uefi::prelude::*;
use uefi::table::boot::MemoryType;

pub fn run_memory_test(st: &mut SystemTable<Boot>) -> MemTestResult {
    let _ =
        st.stdout().output_string(uefi::cstr16!("  [MEMTEST] Starting memory diagnostics...\r\n"));
    let bs = st.boot_services();
    let mmap_size = bs.memory_map_size().map_size + 2048;
    let mut result =
        MemTestResult { total_mb: 0, usable_mb: 0, tested_regions: 0, errors_found: 0 };
    if let Ok(buffer) = bs.allocate_pool(MemoryType::LOADER_DATA, mmap_size) {
        let slice = unsafe { core::slice::from_raw_parts_mut(buffer, mmap_size) };
        if let Ok(mmap) = bs.memory_map(slice) {
            for desc in mmap.entries() {
                result.total_mb += (desc.page_count * 4096) / (1024 * 1024);
                if desc.ty == MemoryType::CONVENTIONAL {
                    result.usable_mb += (desc.page_count * 4096) / (1024 * 1024);
                    if desc.phys_start >= MIN_TEST_ADDR && desc.page_count > 0 {
                        result.tested_regions += 1;
                        result.errors_found += test_region(desc.phys_start, desc.page_count);
                    }
                }
            }
        }
        let _ = bs.free_pool(buffer);
    }
    print_results(st, &result);
    result
}
