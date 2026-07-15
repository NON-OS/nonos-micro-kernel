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

// Unmap an MMIO range previously installed by `map_user_mmio`. The
// range must lie in the active address space; broker grant
// revocation calls this from the holder pid's exit context or from
// the device-release syscall, both of which run with the correct
// CR3 active. The unmap path emits a per-asid SMP TLB shootdown.
pub fn unmap_user_mmio(virtual_addr: VirtAddr, size: usize) -> PagingResult<()> {
    for i in 0..pages_needed(size) {
        let va = VirtAddr::new(virtual_addr.as_u64() + (i * PAGE_SIZE_4K) as u64);
        let _ = unmap_page(va);
    }
    Ok(())
}
