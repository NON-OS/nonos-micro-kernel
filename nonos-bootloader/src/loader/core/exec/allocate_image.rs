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

extern crate alloc;

use alloc::format;
use uefi::table::boot::{AllocateType, BootServices, MemoryType};

use crate::loader::errors::{LoaderError, LoaderResult};
use crate::log::logger::{log_error, log_info};

use crate::loader::core::alloc::record_alloc;
use crate::loader::core::constants::MAX_ALLOCS;

// Reserve a contiguous physical range that backs the kernel image.
// Upper-half kernels declare a virt window only and let the boot
// loader pick any phys range; legacy ET_EXEC images keep their
// `AllocateType::Address(virt_min)` placement so phys == virt for
// the rest of the boot. Returns the physical base address.
pub fn allocate_image(
    bs: &BootServices,
    virt_min: u64,
    pages_needed: usize,
    upper_half: bool,
    allocations: &mut [(u64, usize); MAX_ALLOCS],
    alloc_count: &mut usize,
) -> LoaderResult<u64> {
    let alloc_type =
        if upper_half { AllocateType::AnyPages } else { AllocateType::Address(virt_min) };

    match bs.allocate_pages(alloc_type, MemoryType::LOADER_DATA, pages_needed) {
        Ok(addr) => {
            record_alloc(allocations, alloc_count, addr, pages_needed)?;
            log_info(
                "loader",
                &format!(
                    "Allocated {} pages at phys 0x{:x} (ET_EXEC, {})",
                    pages_needed,
                    addr,
                    if upper_half { "upper-half" } else { "low-half" },
                ),
            );
            Ok(addr)
        }
        Err(e) => {
            let target = if upper_half { 0 } else { virt_min };
            log_error(
                "loader",
                &format!("Allocation failed (target=0x{:x}): {:?}", target, e.status()),
            );
            Err(LoaderError::AllocationFailed {
                addr: target,
                pages: pages_needed,
                status: e.status(),
            })
        }
    }
}
