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

use super::super::core::PagingManager;
use crate::memory::paging::error::{PagingError, PagingResult};

impl PagingManager {
    pub fn switch_address_space(&mut self, asid: u32) -> PagingResult<()> {
        let address_space =
            self.address_spaces.get(&asid).ok_or(PagingError::AddressSpaceNotFound)?;
        // `cr3_value` was produced by this manager's `create_address_space`
        // and stays valid until `cleanup_address_space(asid)` removes it.
        //
        // It does not pack the asid, whatever the previous comment here said:
        // `AddressSpace::new` stores the raw page-table frame, so the low bits
        // are zero and the mask below always yields 0. The register is loaded
        // untagged, which is correct but means tagged invalidation does
        // nothing for the switch. The asid is still tracked on the CPU below,
        // where the shootdown broadcaster does use it.
        let root = address_space.cr3_value.as_u64();
        crate::arch::paging::write_root(root, (root & 0xFFF) as u16);
        self.active_page_table = Some(address_space.cr3_value);
        self.active_asid = Some(asid);
        // Record on the calling CPU which asid is now executing.
        // The TLB shootdown broadcaster reads this to scope per-asid
        // invalidations to the cores actually running that CR3.
        crate::smp::percpu::set_active_asid(asid);
        Ok(())
    }
}
