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

//! Backing a span of a guest's address space with fresh frames.

use crate::memory::addr::VirtAddr;
use crate::memory::paging::manager::{map_page_in_asid, translate_in_asid};
use crate::memory::paging::types::PagePermissions;
use crate::syscall::microkernel::errnos::{ERRNO_INVAL, ERRNO_NOMEM};

use super::peer_guard::{in_user_half, supervised_asid, MAX_SPAN, PAGE, PROT_EXEC, PROT_WRITE};

fn span_ok(addr: u64, len: u64) -> bool {
    len != 0 && len <= MAX_SPAN && addr % PAGE == 0 && in_user_half(addr, len)
}

pub(super) fn perms_of(prot: u64) -> PagePermissions {
    let mut perms = PagePermissions::READ | PagePermissions::USER;
    if prot & PROT_WRITE != 0 {
        perms = perms | PagePermissions::WRITE;
    }
    if prot & PROT_EXEC != 0 {
        perms = perms | PagePermissions::EXECUTE;
    }
    perms
}

/// `MkPeerMap`: map `[addr, addr + len)` in a guest the caller supervises.
pub fn sys_peer_map(pid: u64, addr: u64, len: u64, prot: u64) -> i64 {
    let Some(caller) = crate::process::current_pid() else {
        return ERRNO_INVAL;
    };
    let (asid, _held) = match supervised_asid(caller, pid) {
        Ok(pair) => pair,
        Err(e) => return e,
    };
    if !span_ok(addr, len) {
        return ERRNO_INVAL;
    }
    let perms = perms_of(prot);
    for i in 0..len.div_ceil(PAGE) {
        let va = VirtAddr::new(addr + i * PAGE);
        if translate_in_asid(asid, va).is_some() {
            continue;
        }
        let Some(frame) = crate::memory::frame_alloc::allocate_frame() else {
            return ERRNO_NOMEM;
        };
        crate::memory::frame_alloc::zero_frame(frame);
        if map_page_in_asid(asid, va, frame, perms).is_err() {
            let _ = crate::memory::frame_alloc::deallocate_frame(frame);
            return ERRNO_NOMEM;
        }
    }
    0
}
