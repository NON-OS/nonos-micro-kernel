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

//! Moving bytes between a supervisor and a guest it created.

use crate::memory::addr::VirtAddr;
use crate::memory::paging::manager::translate_in_asid;
use crate::syscall::microkernel::errnos::{ERRNO_FAULT, ERRNO_INVAL};

use super::peer_chunk::{chunk_copy, validate};
use super::peer_guard::{in_user_half, supervised_asid, MAX_SPAN, PAGE};

/// `MkPeerCopy`: `to_guest` chooses the direction; returns bytes moved.
pub fn sys_peer_copy(pid: u64, guest_addr: u64, buf: u64, len: u64, to_guest: u64) -> i64 {
    let Some(caller) = crate::process::current_pid() else {
        return ERRNO_INVAL;
    };
    let (asid, _held) = match supervised_asid(caller, pid) {
        Ok(pair) => pair,
        Err(e) => return e,
    };
    if len == 0 || len > MAX_SPAN || !in_user_half(guest_addr, len) {
        return ERRNO_INVAL;
    }
    let writing = to_guest != 0;
    if validate(buf, len, writing).is_err() {
        return ERRNO_FAULT;
    }
    let mut done = 0u64;
    while done < len {
        let va = guest_addr + done;
        let page = va & !(PAGE - 1);
        let offset = va - page;
        let chunk = core::cmp::min(PAGE - offset, len - done);
        let Some(phys) = translate_in_asid(asid, VirtAddr::new(page)) else {
            return ERRNO_FAULT;
        };
        if chunk_copy(phys, offset, buf + done, chunk, writing).is_err() {
            return ERRNO_FAULT;
        }
        done += chunk;
    }
    done as i64
}
