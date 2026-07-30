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

pub fn unmap_page(virtual_addr: VirtAddr) -> PagingResult<PhysAddr> {
    let (phys, perms, size) =
        without_interrupts(|| PAGING_MANAGER.lock().unmap_page(virtual_addr))?;
    PAGING_STATS.record_unmapping(perms, size);
    Ok(phys)
}
