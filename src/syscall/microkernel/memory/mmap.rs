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
use core::sync::atomic::{AtomicU32, Ordering};

use super::consts::{is_user_space, MAX_MMAP_SIZE, PAGE_SIZE, PROT_EXEC, PROT_WRITE};
use super::va::{release_va, reserve_va, rollback_mapped_pages};

static MMAP_TRACE_COUNT: AtomicU32 = AtomicU32::new(0);

fn is_traced(pid: u32) -> bool {
    matches!(pid, 0x18 | 0x1a | 0x1b)
}

fn trace(label: &[u8], pid: u32, value: u64) {
    if !is_traced(pid) || MMAP_TRACE_COUNT.fetch_add(1, Ordering::Relaxed) >= 48 {
        return;
    }
    crate::sys::serial::print(b"[MMAP] ");
    crate::sys::serial::print(label);
    crate::sys::serial::print(b" pid=");
    crate::sys::serial::print_hex(pid as u64);
    crate::sys::serial::print(b" v=");
    crate::sys::serial::print_hex(value);
    crate::sys::serial::println(b"");
}

pub fn sys_mmap(addr: u64, length: usize, prot: u32, _flags: u32) -> i64 {
    let pid = current_pid().unwrap_or(0);
    trace(b"enter", pid, length as u64);
    if length == 0 || length > MAX_MMAP_SIZE {
        return ERRNO_INVAL;
    }
    if addr != 0 && !is_user_space(addr, length) {
        return ERRNO_PERM;
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

    for i in 0..pages as usize {
        let va = VirtAddr::new(base + (i * PAGE_SIZE) as u64);
        let frame = match crate::memory::frame_alloc::allocate_frame() {
            Some(pa) => pa,
            None => {
                rollback_mapped_pages(base, i);
                if allocator_owned {
                    let _ = release_va(base, pages);
                }
                trace(b"oom", pid, base);
                return ERRNO_NOMEM;
            }
        };
        if map_page(va, frame, perms).is_err() {
            let _ = crate::memory::frame_alloc::deallocate_frame(frame);
            rollback_mapped_pages(base, i);
            if allocator_owned {
                let _ = release_va(base, pages);
            }
            trace(b"mapfail", pid, base);
            return ERRNO_NOMEM;
        }
    }
    trace(b"ok", pid, base);
    base as i64
}
