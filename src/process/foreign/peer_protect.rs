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

use crate::memory::addr::VirtAddr;
use crate::memory::paging::manager::{map_page_in_asid, translate_in_asid};
use crate::syscall::microkernel::errnos::{ERRNO_FAULT, ERRNO_INVAL};

use super::peer_guard::{in_user_half, supervised_asid, MAX_SPAN, PAGE};
use super::peer_map::perms_of;

fn span_ok(addr: u64, len: u64) -> bool {
    len != 0 && len <= MAX_SPAN && addr % PAGE == 0 && in_user_half(addr, len)
}

/// `MkPeerProtect`: set the protection of `[addr, addr + len)` in a guest the
/// caller supervises.
pub fn sys_peer_protect(pid: u64, addr: u64, len: u64, prot: u64) -> i64 {
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
        let Some(phys) = translate_in_asid(asid, va) else {
            return ERRNO_FAULT;
        };
        if map_page_in_asid(asid, va, phys, perms).is_err() {
            return ERRNO_FAULT;
        }
    }
    0
}
