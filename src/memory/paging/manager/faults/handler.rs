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

use crate::memory::addr::VirtAddr;

use super::super::core::PagingManager;
use crate::memory::paging::constants::*;
use crate::memory::paging::error::{PagingError, PagingResult};
use crate::memory::paging::stats::PagingStatistics;

impl PagingManager {
    pub fn handle_page_fault(
        &mut self,
        virtual_addr: VirtAddr,
        error_code: u64,
        stats: &PagingStatistics,
    ) -> PagingResult<()> {
        stats.record_page_fault();

        if error_code & PF_WRITE != 0 && error_code & PF_PRESENT != 0 {
            stats.record_cow_fault();
            return self.handle_cow_fault(virtual_addr, stats);
        }

        if error_code & PF_PRESENT == 0 {
            if error_code & PF_INSTRUCTION != 0 {
                return Err(PagingError::UnhandledPageFault);
            }
            stats.record_demand_load();
            return self.handle_demand_fault(virtual_addr, stats);
        }

        Err(PagingError::UnhandledPageFault)
    }
}
