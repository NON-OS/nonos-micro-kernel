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

//! `MkDmaMap` / `MkDmaUnmap` handlers. Cap-gated by `Capability::Dma`
//! at the contract layer. Translate the syscall arguments into a
//! broker DMA grant request, write the result back into the
//! caller's buffer, and roll the grant back if that copy fails.

use core::mem::size_of;
use core::sync::atomic::{AtomicU32, Ordering};

use super::errnos::{
    ERRNO_FAULT, ERRNO_INVAL, ERRNO_NODEV, ERRNO_NOMEM, ERRNO_NOTSUP, ERRNO_PERM, ERRNO_STALE,
};
use crate::hardware::broker::{DmaError, DmaMapError, DmaMapRequest};
use crate::process::current_pid;
use crate::usercopy::{validate_user_write, write_user_value};

static DMA_UNMAP_TRACE_COUNT: AtomicU32 = AtomicU32::new(0);

fn trace_unmap(label: &[u8], pid: u32) {
    if pid != 7 || DMA_UNMAP_TRACE_COUNT.fetch_add(1, Ordering::Relaxed) >= 24 {
        return;
    }
    crate::sys::serial::print(b"[DMA-UNMAP] ");
    crate::sys::serial::println(label);
}

#[repr(C)]
#[derive(Clone, Copy)]
struct DmaMapOut {
    user_va: u64,
    device_addr: u64,
    length: u64,
    grant_id: u64,
}

const _: () = assert!(size_of::<DmaMapOut>() == 32);

pub fn sys_dma_map(device_id: u64, claim_epoch: u64, length: u64, flags: u32, out_ptr: u64) -> i64 {
    let pid = match current_pid() {
        Some(p) => p,
        None => return ERRNO_PERM,
    };
    if out_ptr == 0 {
        return ERRNO_FAULT;
    }
    if validate_user_write(out_ptr, size_of::<DmaMapOut>()).is_err() {
        return ERRNO_FAULT;
    }
    let req = DmaMapRequest { device_id, claim_epoch, length, flags };
    let r = match crate::hardware::broker::dma_map_for_caller(pid, req) {
        Ok(r) => r,
        Err(e) => return errno_for(e),
    };
    let out = DmaMapOut {
        user_va: r.user_va,
        device_addr: r.device_addr,
        length: r.length,
        grant_id: r.grant_id,
    };
    if write_user_value(out_ptr, &out).is_err() {
        let _ = crate::hardware::broker::dma_unmap_grant(pid, r.grant_id);
        return ERRNO_FAULT;
    }
    0
}

pub fn sys_dma_unmap(grant_id: u64) -> i64 {
    let pid = match current_pid() {
        Some(p) => p,
        None => return ERRNO_PERM,
    };
    trace_unmap(b"enter", pid);
    match crate::hardware::broker::dma_unmap_grant(pid, grant_id) {
        Ok(()) => {
            trace_unmap(b"ok", pid);
            0
        }
        Err(DmaError::NotHolder) => ERRNO_PERM,
        Err(DmaError::UnknownGrant) => ERRNO_INVAL,
    }
}

fn errno_for(e: DmaMapError) -> i64 {
    match e {
        DmaMapError::NotClaimed => ERRNO_PERM,
        DmaMapError::StaleEpoch => ERRNO_STALE,
        DmaMapError::UnknownDevice => ERRNO_NODEV,
        DmaMapError::BadAlignment | DmaMapError::BadLength | DmaMapError::BadLengthForClass => {
            ERRNO_INVAL
        }
        DmaMapError::UnsupportedFlags => ERRNO_NOTSUP,
        DmaMapError::NoMemory | DmaMapError::NoVaSpace | DmaMapError::MapFailed => ERRNO_NOMEM,
    }
}
