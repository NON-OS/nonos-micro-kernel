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

use crate::memory::addr::{PhysAddr, VirtAddr};

use super::super::core::PagingManager;
use crate::memory::paging::error::{PagingError, PagingResult};
use crate::memory::paging::stats::PagingStatistics;
use crate::memory::paging::types::{PagePermissions, PageSize};

impl PagingManager {
    pub fn unmap_page_in_asid(
        &mut self,
        asid: u32,
        virtual_addr: VirtAddr,
        permissions: PagePermissions,
        size: PageSize,
        stats: &PagingStatistics,
    ) -> PagingResult<PhysAddr> {
        if !self.initialized {
            return Err(PagingError::NotInitialized);
        }
        let physical_addr = self.remove_mapping_in_asid(asid, virtual_addr)?;
        stats.record_unmapping(permissions, size);
        Ok(physical_addr)
    }
}
