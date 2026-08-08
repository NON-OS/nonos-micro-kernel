// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use super::core::MMU;
use crate::memory::addr::VirtAddr;

impl MMU {
    /// Drop every non-global TLB entry, by reloading the page-table root on
    /// x86_64 and by the broadcast invalidate on aarch64.
    pub fn invalidate_tlb_all(&self) {
        crate::arch::paging::invalidate_all();
    }

    /// Drop the entry covering one page.
    pub fn invalidate_tlb_page(&self, virt_addr: VirtAddr) {
        crate::arch::paging::invalidate_page(virt_addr.as_u64());
    }
}
