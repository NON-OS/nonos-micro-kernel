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

use super::map_page;
use crate::memory::addr::{PhysAddr, VirtAddr};
use crate::memory::paging::error::PagingResult;
use crate::memory::paging::types::PagePermissions;

pub fn map_user_page(
    virtual_addr: VirtAddr,
    physical_addr: PhysAddr,
    writable: bool,
) -> PagingResult<()> {
    let mut permissions = PagePermissions::READ | PagePermissions::USER;
    if writable {
        permissions = permissions | PagePermissions::WRITE;
    }
    map_page(virtual_addr, physical_addr, permissions)
}
