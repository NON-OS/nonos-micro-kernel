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

#[repr(C)]
pub struct LinkMapEntry {
    pub l_addr: u64,
    pub l_name: *const u8,
    pub l_ld: u64,
    pub l_next: *mut LinkMapEntry,
    pub l_prev: *mut LinkMapEntry,
}

impl LinkMapEntry {
    pub fn new(base_addr: u64, name_ptr: *const u8, dynamic_addr: u64) -> Self {
        Self {
            l_addr: base_addr,
            l_name: name_ptr,
            l_ld: dynamic_addr,
            l_next: core::ptr::null_mut(),
            l_prev: core::ptr::null_mut(),
        }
    }
}
