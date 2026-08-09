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

use super::globals::{PAGING_MANAGER, PAGING_STATS};
use crate::arch::x86_64::idt::without_interrupts;
use crate::memory::addr::{PhysAddr, VirtAddr};
use crate::memory::paging::error::PagingResult;
use crate::memory::paging::types::{PagePermissions, PageSize};

pub fn map_page_in_asid(
    asid: u32,
    virtual_addr: VirtAddr,
    physical_addr: PhysAddr,
    permissions: PagePermissions,
) -> PagingResult<()> {
    without_interrupts(|| {
        PAGING_MANAGER.lock().map_page_in_asid(
            asid,
            virtual_addr,
            physical_addr,
            permissions,
            PageSize::Size4KiB,
            &PAGING_STATS,
        )
    })
}

pub fn unmap_page_in_asid(
    asid: u32,
    virtual_addr: VirtAddr,
    permissions: PagePermissions,
) -> PagingResult<PhysAddr> {
    without_interrupts(|| {
        PAGING_MANAGER.lock().unmap_page_in_asid(
            asid,
            virtual_addr,
            permissions,
            PageSize::Size4KiB,
            &PAGING_STATS,
        )
    })
}

pub fn translate_in_asid(asid: u32, virtual_addr: VirtAddr) -> Option<PhysAddr> {
    PAGING_MANAGER.lock().translate_in_asid(asid, virtual_addr)
}
