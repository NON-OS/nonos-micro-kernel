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

use super::{map_page, unmap_page};
use crate::memory::addr::{PhysAddr, VirtAddr};
use crate::memory::paging::constants::{pages_needed, PAGE_SIZE_4K};
use crate::memory::paging::error::PagingResult;
use crate::memory::paging::types::PagePermissions;

// Map a DMA buffer into the currently active user address space.
// Permissions: user, read+write, no-execute, write-back cacheable.
// On x86_64 PCI devices snoop the cache, so write-back is the
// correct cache mode for coherent DMA; uncached / write-combining
// would be wrong here. Caller is the hardware broker; no other
// path is allowed to expose physical memory to a capsule.
pub fn map_user_dma(
    virtual_addr: VirtAddr,
    physical_addr: PhysAddr,
    size: usize,
) -> PagingResult<()> {
    let permissions = PagePermissions::USER | PagePermissions::READ | PagePermissions::WRITE;
    let pages = pages_needed(size);
    for i in 0..pages {
        let va = VirtAddr::new(virtual_addr.as_u64() + (i * PAGE_SIZE_4K) as u64);
        let pa = PhysAddr::new(physical_addr.as_u64() + (i * PAGE_SIZE_4K) as u64);
        if let Err(e) = map_page(va, pa, permissions) {
            for j in 0..i {
                let rb = VirtAddr::new(virtual_addr.as_u64() + (j * PAGE_SIZE_4K) as u64);
                let _ = unmap_page(rb);
            }
            return Err(e);
        }
    }
    Ok(())
}
