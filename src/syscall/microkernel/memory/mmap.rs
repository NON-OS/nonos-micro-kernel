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

use crate::memory::paging::{map_page, PagePermissions};
use crate::memory::VirtAddr;
use crate::process::current_pid;
use crate::syscall::microkernel::errnos::{ERRNO_INVAL, ERRNO_NOMEM, ERRNO_PERM};

use super::accounting::record_mmap;
use super::consts::{is_user_space, MAX_MMAP_SIZE, PAGE_SIZE, PROT_EXEC, PROT_WRITE};
use super::va::{release_va, reserve_va, rollback_mapped_pages};
pub fn sys_mmap(addr: u64, length: usize, prot: u32, _flags: u32) -> i64 {
    let pid = current_pid().unwrap_or(0);
    if length == 0 || length > MAX_MMAP_SIZE {
        return ERRNO_INVAL;
    }
    if addr != 0 && !is_user_space(addr, length) {
        return ERRNO_PERM;
    }
    // A fixed address must be page aligned; an unaligned hint would otherwise
    // be mapped at the containing page yet returned verbatim.
    if addr != 0 && (addr & (PAGE_SIZE as u64 - 1)) != 0 {
        return ERRNO_INVAL;
    }
    let pages = ((length + PAGE_SIZE - 1) / PAGE_SIZE) as u64;
    let mut perms = PagePermissions::READ | PagePermissions::USER;
    if prot & PROT_WRITE != 0 {
        perms = perms | PagePermissions::WRITE;
    }
    if prot & PROT_EXEC != 0 {
        perms = perms | PagePermissions::EXECUTE;
    }
    let allocator_owned = addr == 0;
    let base = if allocator_owned {
        match reserve_va(pages) {
            Some(b) => b,
            None => return ERRNO_NOMEM,
        }
    } else {
        addr
    };
    // Refuse to map over an already-present page in the fixed-address case:
    // overwriting the PTE would orphan the previous frame and corrupt the
    // caller's own address space. Allocator-chosen ranges are always fresh.
    if !allocator_owned {
        for i in 0..pages as usize {
            if crate::memory::paging::is_mapped(VirtAddr::new(base + (i * PAGE_SIZE) as u64)) {
                return ERRNO_INVAL;
            }
        }
    }
    for i in 0..pages as usize {
        let va = VirtAddr::new(base + (i * PAGE_SIZE) as u64);
        let frame = match crate::memory::frame_alloc::allocate_frame() {
            Some(pa) => pa,
            None => {
                rollback_mapped_pages(base, i);
                if allocator_owned && !release_va(base, pages) {
                    crate::sys::serial::println(b"[MMAP] release_va_failed");
                }
                return ERRNO_NOMEM;
            }
        };
        if map_page(va, frame, perms).is_err() {
            if crate::memory::frame_alloc::deallocate_frame(frame).is_err() {
                crate::sys::serial::println(b"[MMAP] frame_release_failed");
            }
            rollback_mapped_pages(base, i);
            if allocator_owned && !release_va(base, pages) {
                crate::sys::serial::println(b"[MMAP] release_va_failed");
            }
            return ERRNO_NOMEM;
        }
        // POSIX mmap hands back zeroed memory. The frame allocator only zeroes
        // on free, so a first-use frame still holds stale RAM; zero it through
        // the directmap so callers (notably the std PAL dlmalloc heap) never
        // read garbage as live data — that was corrupting dlmalloc chunk
        // headers and sending capsules into an allocator spin.
        unsafe {
            let dm = (crate::memory::layout::DIRECTMAP_BASE + frame.as_u64()) as *mut u8;
            core::ptr::write_bytes(dm, 0, PAGE_SIZE);
        }
    }
    record_mmap(pid, length, base);
    base as i64
}
