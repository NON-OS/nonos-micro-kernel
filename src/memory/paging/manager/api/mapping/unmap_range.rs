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

use super::unmap_page;
use crate::memory::addr::VirtAddr;
use crate::memory::paging::constants::{pages_needed, PAGE_SIZE_4K};
use crate::memory::paging::error::PagingResult;

// Multi-page unmap. Walks 4 KiB pages from `virtual_addr` for `size`
// bytes (rounded up). Stops at the first failure and returns it.
pub fn unmap_range(virtual_addr: VirtAddr, size: usize) -> PagingResult<()> {
    for i in 0..pages_needed(size) {
        let va = VirtAddr::new(virtual_addr.as_u64() + (i * PAGE_SIZE_4K) as u64);
        unmap_page(va)?;
    }
    Ok(())
}
