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

use super::super::globals::{PAGING_MANAGER, PAGING_STATS};
use crate::arch::run_without_interrupts as without_interrupts;
use crate::memory::addr::{PhysAddr, VirtAddr};
use crate::memory::paging::error::PagingResult;
use crate::memory::paging::types::{PagePermissions, PageSize};

pub fn map_page(
    virtual_addr: VirtAddr,
    physical_addr: PhysAddr,
    permissions: PagePermissions,
) -> PagingResult<()> {
    // Disable interrupts across the PAGING_MANAGER critical section.
    // Without this, a timer ISR firing on this CPU while the lock is
    // held would try to call switch_to_process_address_space from
    // preempt_current_process and re-enter the same spin::Mutex,
    // deadlocking the CPU.
    without_interrupts(|| {
        PAGING_MANAGER.lock().map_page(
            virtual_addr,
            physical_addr,
            permissions,
            PageSize::Size4KiB,
            &PAGING_STATS,
        )
    })
}
