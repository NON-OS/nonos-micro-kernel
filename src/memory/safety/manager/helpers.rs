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

use crate::memory::layout;

pub(super) fn get_timestamp() -> u64 {
    crate::arch::read_time_counter()
}

pub(super) fn is_guard_compromised(addr: u64, size: u64) -> bool {
    if addr == 0 || size == 0 {
        return true;
    }
    let mut current_addr = addr;
    while current_addr < addr + size {
        if crate::memory::paging::translate_address(crate::memory::addr::VirtAddr::new(
            current_addr,
        ))
        .is_some()
        {
            return true;
        }
        current_addr += layout::PAGE_SIZE as u64;
    }
    false
}
