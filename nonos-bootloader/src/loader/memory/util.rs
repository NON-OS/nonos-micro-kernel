// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use crate::loader::types::memory;

pub unsafe fn zero_memory(addr: u64, size: usize) {
    if size > 0 {
        core::ptr::write_bytes(addr as *mut u8, 0, size);
    }
}

pub unsafe fn copy_memory(src: *const u8, dst: u64, size: usize) {
    if size > 0 {
        core::ptr::copy_nonoverlapping(src, dst as *mut u8, size);
    }
}

pub fn pages_for_size(size: usize) -> usize {
    memory::pages_needed(size)
}
pub fn page_align_down(addr: u64) -> u64 {
    memory::page_align_down(addr)
}
pub fn page_align_up(addr: u64) -> u64 {
    memory::page_align_up(addr)
}
pub fn is_page_aligned(addr: u64) -> bool {
    (addr & (memory::PAGE_SIZE as u64 - 1)) == 0
}
