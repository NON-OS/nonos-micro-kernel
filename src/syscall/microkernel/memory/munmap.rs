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

use crate::memory::paging::unmap_page;
use crate::memory::VirtAddr;
use crate::process::current_pid;
use crate::syscall::microkernel::errnos::ERRNO_INVAL;

use super::accounting::record_munmap;
use super::consts::{is_user_space, MAX_MMAP_SIZE, PAGE_SIZE};
use super::va::release_va;

pub fn sys_munmap(addr: u64, length: usize) -> i64 {
    if addr == 0 || length == 0 {
        return ERRNO_INVAL;
    }
    if addr % PAGE_SIZE as u64 != 0 {
        return ERRNO_INVAL;
    }
    // Bound the range to user space, symmetric with sys_mmap. Without this an
    // out-of-range or oversized request would drive unmap_page/deallocate_frame
    // over kernel addresses, and addr + i * PAGE_SIZE below could wrap.
    if length > MAX_MMAP_SIZE || !is_user_space(addr, length) {
        return ERRNO_INVAL;
    }
    let pages = ((length + PAGE_SIZE - 1) / PAGE_SIZE) as u64;
    for i in 0..pages as usize {
        let va = VirtAddr::new(addr + (i * PAGE_SIZE) as u64);
        if let Ok(phys) = unmap_page(va) {
            if crate::memory::frame_alloc::deallocate_frame(phys).is_err() {
                crate::sys::serial::println(b"[MUNMAP] frame_release_failed");
            }
        }
    }
    if !release_va(addr, pages) {
        crate::sys::serial::println(b"[MUNMAP] release_va_failed");
    }
    record_munmap(current_pid().unwrap_or(0), length, addr);
    0
}
