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


//! Changing the protection of pages a guest already has.
//!
//! A supervisor that loads code cannot know the final protection when it
//! lays the pages down. A dynamic linker maps a library writable, applies
//! relocations to it, and only then asks for it to be executable. Without
//! this the pages would stay writable and the program would fault on its
//! first call into the library, or the supervisor would have to map the
//! code writable and executable at once, which the mapping layer refuses
//! and should go on refusing.

use crate::memory::addr::VirtAddr;
use crate::memory::paging::manager::{map_page_in_asid, translate_in_asid};
use crate::syscall::microkernel::errnos::{ERRNO_FAULT, ERRNO_INVAL};

use super::peer_guard::{supervised_asid, MAX_SPAN, PAGE};
use super::peer_map::perms_of;

fn span_ok(addr: u64, len: u64) -> bool {
    len != 0 && len <= MAX_SPAN && addr % PAGE == 0 && addr.checked_add(len).is_some()
}

/// `MkPeerProtect`: set the protection of `[addr, addr + len)` in a guest
/// the caller supervises. Every page must already be mapped; a hole is an
/// error rather than a silent gap, because a caller asking for execute on
/// a range it has not filled is not asking for what it thinks.
pub fn sys_peer_protect(pid: u64, addr: u64, len: u64, prot: u64) -> i64 {
    let Some(caller) = crate::process::current_pid() else {
        return ERRNO_INVAL;
    };
    let asid = match supervised_asid(caller, pid as u32) {
        Ok(a) => a,
        Err(e) => return e,
    };
    if !span_ok(addr, len) {
        return ERRNO_INVAL;
    }
    let perms = perms_of(prot);
    for i in 0..len.div_ceil(PAGE) {
        let va = VirtAddr::new(addr + i * PAGE);
        let Some(phys) = translate_in_asid(asid, va) else {
            return ERRNO_FAULT;
        };
        if map_page_in_asid(asid, va, phys, perms).is_err() {
            return ERRNO_FAULT;
        }
    }
    0
}
