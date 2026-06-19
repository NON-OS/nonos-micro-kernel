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
use crate::memory::paging::manager::api::translate_address;

pub(super) fn rip_byte_range_mapped(rip: u64, len: u64) -> bool {
    if len == 0 {
        return true;
    }
    translate_address(VirtAddr::new(rip)).is_some()
        && translate_address(VirtAddr::new(rip.wrapping_add(len - 1))).is_some()
}
