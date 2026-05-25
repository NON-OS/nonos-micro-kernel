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

use core::ptr;

pub(super) unsafe fn write_u64(target_addr: u64, value: u64) {
    unsafe { ptr::write(target_addr as *mut u64, value) }
}

pub(super) unsafe fn copy_symbol(target_addr: u64, source_addr: u64, size: usize) {
    unsafe { ptr::copy_nonoverlapping(source_addr as *const u8, target_addr as *mut u8, size) }
}
