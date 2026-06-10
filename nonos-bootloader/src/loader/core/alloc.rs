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

use super::constants::MAX_ALLOCS;
use crate::loader::errors::{LoaderError, LoaderResult};
use crate::log::logger::{log_error, log_info};
use alloc::format;

pub fn record_alloc(
    table: &mut [(u64, usize); MAX_ALLOCS],
    count: &mut usize,
    addr: u64,
    pages: usize,
) -> LoaderResult<()> {
    if *count >= MAX_ALLOCS {
        return Err(LoaderError::AllocationTableFull);
    }
    table[*count] = (addr, pages);
    *count += 1;
    Ok(())
}

pub fn free_all(
    bs: &uefi::table::boot::BootServices,
    table: &[(u64, usize); MAX_ALLOCS],
    count: usize,
) {
    for i in 0..count {
        let (addr, pages) = table[i];
        if addr == 0 || pages == 0 {
            continue;
        }
        match bs.free_pages(addr, pages) {
            Ok(_) => log_info("loader", &format!("Freed pages at 0x{:x} ({} pages)", addr, pages)),
            Err(e) => log_error(
                "loader",
                &format!("free_pages failed for 0x{:x} ({}): {:?}", addr, pages, e.status()),
            ),
        }
    }
}
