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

use super::super::paging::manager;
use crate::memory::addr::{PhysAddr, VirtAddr};

#[inline]
pub fn translate_virtual(va: VirtAddr) -> Option<PhysAddr> {
    manager::translate_address(va)
}

#[inline]
pub fn is_address_mapped(va: VirtAddr) -> bool {
    manager::is_mapped(va)
}

pub fn handle_unified_page_fault(
    fault_addr: VirtAddr,
    error_code: u64,
) -> Result<(), &'static str> {
    manager::handle_page_fault(fault_addr, error_code).map_err(|_| "Page fault handling failed")
}
