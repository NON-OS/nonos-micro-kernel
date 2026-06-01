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

use core::mem::size_of;

use crate::hardware::broker::{self, MmioMapRequest};
use crate::process::current_pid;
use crate::syscall::microkernel::errnos::{ERRNO_FAULT, ERRNO_INVAL, ERRNO_PERM};
use crate::usercopy::{validate_user_write, write_user_value};

use super::errno_map::errno_for;

#[repr(C)]
#[derive(Clone, Copy)]
struct MmioMapOut {
    user_va: u64,
    length: u64,
    grant_id: u64,
}

const _: () = assert!(size_of::<MmioMapOut>() == 24);

pub fn sys_mmio_map(
    device_id: u64,
    claim_epoch: u64,
    bar_index: u32,
    offset: u64,
    length: u64,
    flags: u32,
    out_ptr: u64,
) -> i64 {
    let pid = match current_pid() {
        Some(p) => p,
        None => return ERRNO_PERM,
    };
    if out_ptr == 0 {
        return ERRNO_FAULT;
    }
    if validate_user_write(out_ptr, size_of::<MmioMapOut>()).is_err() {
        return ERRNO_FAULT;
    }
    if bar_index > u8::MAX as u32 {
        return ERRNO_INVAL;
    }
    let req = MmioMapRequest {
        device_id,
        claim_epoch,
        bar_index: bar_index as u8,
        offset,
        length,
        flags,
    };
    crate::sys::serial::println(b"[MMIO] broker");
    let result = match broker::map_for_caller(pid, req) {
        Ok(r) => r,
        Err(e) => {
            crate::sys::serial::print(b"[MMIO] err=");
            crate::sys::serial::print_dec(errno_for(e) as u64);
            crate::sys::serial::println(b"");
            return errno_for(e);
        }
    };
    crate::sys::serial::println(b"[MMIO] broker ok");
    let out =
        MmioMapOut { user_va: result.user_va, length: result.length, grant_id: result.grant_id };
    crate::sys::serial::println(b"[MMIO] out");
    if write_user_value(out_ptr, &out).is_err() {
        // Pages are mapped but the caller can't read the result. Roll
        // the grant back so we don't leak a window with no handle.
        if broker::unmap_grant(pid, result.grant_id).is_err() {
            crate::sys::serial::println(b"[MMIO] rollback failed");
        }
        return ERRNO_FAULT;
    }
    0
}
