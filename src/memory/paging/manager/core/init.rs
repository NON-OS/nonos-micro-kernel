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

use super::types::PagingManager;
use crate::arch::paging::{descriptor, read_root};
use crate::memory::addr::PhysAddr;
use crate::memory::paging::error::PagingResult;

impl PagingManager {
    pub fn init(&mut self) -> PagingResult<()> {
        if self.initialized {
            return Ok(());
        }
        // Adopt whatever table the boot path left installed. The low bits of
        // the root register carry an ASID or PCID rather than address, so they
        // are masked off the same way a descriptor's are.
        self.active_page_table = Some(PhysAddr::new(descriptor::address(read_root())));
        crate::sys::serial::print(b"[VM] adopted root ");
        crate::sys::serial::print_hex(descriptor::address(read_root()));
        crate::sys::serial::println(b"");
        self.initialized = true;
        self.create_kernel_address_space()?;
        Ok(())
    }
}
