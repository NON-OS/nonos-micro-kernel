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

//! Device enumerate / claim / release. Cap-gated at the contract
//! layer: `MkDeviceList` requires `DeviceEnum`, claim and release
//! require `Driver`. Handlers do not re-check the capability;
//! reaching here proves it.

use core::mem::size_of;
use core::sync::atomic::{AtomicU32, Ordering};

use super::errnos::{ERRNO_BUSY, ERRNO_FAULT, ERRNO_INVAL, ERRNO_NODEV, ERRNO_PERM};
use crate::hardware::broker::{self, ClaimError, DeviceRecord};
use crate::process::current_pid;
use crate::usercopy::{validate_user_write, write_user_value};

static DEVICE_RELEASE_TRACE_COUNT: AtomicU32 = AtomicU32::new(0);

fn trace_release(label: &[u8], pid: u32) {
    if pid != 7 || DEVICE_RELEASE_TRACE_COUNT.fetch_add(1, Ordering::Relaxed) >= 24 {
        return;
    }
    crate::sys::serial::print(b"[DEV-RELEASE] ");
    crate::sys::serial::println(label);
}

pub fn sys_device_list(class: u32, buf_ptr: u64, count: u64) -> i64 {
    let snapshot = broker::list_by_class(class);
    let total = snapshot.len();
    if count == 0 {
        return total as i64;
    }
    if buf_ptr == 0 {
        return ERRNO_FAULT;
    }
    let to_write = core::cmp::min(count as usize, total);
    let bytes = match (to_write as u64).checked_mul(size_of::<DeviceRecord>() as u64) {
        Some(v) => v,
        None => return ERRNO_INVAL,
    };
    if validate_user_write(buf_ptr, bytes as usize).is_err() {
        return ERRNO_FAULT;
    }
    let stride = size_of::<DeviceRecord>() as u64;
    for (i, rec) in snapshot.iter().take(to_write).enumerate() {
        let dst = buf_ptr + (i as u64) * stride;
        if write_user_value(dst, rec).is_err() {
            return ERRNO_FAULT;
        }
    }
    to_write as i64
}

pub fn sys_device_claim(device_id: u64) -> i64 {
    let pid = match current_pid() {
        Some(p) => p,
        None => return ERRNO_PERM,
    };
    if !broker::contains(device_id) {
        return ERRNO_NODEV;
    }
    match broker::claim_device(pid, device_id) {
        Ok(epoch) => epoch as i64,
        Err(ClaimError::AlreadyClaimed) => ERRNO_BUSY,
        Err(ClaimError::UnknownDevice) => ERRNO_NODEV,
        Err(ClaimError::NotHolder) | Err(ClaimError::NotClaimed) => ERRNO_INVAL,
    }
}

// Releases the claim on `device_id` held by the calling pid. Any
// outstanding MMIO grants for the device are torn down first; the
// caller's CR3 is active here so the unmap and TLB shootdown are
// in-context. The exit path performs the same cleanup for every
// claim a dying capsule was holding.
pub fn sys_device_release(device_id: u64) -> i64 {
    let pid = match current_pid() {
        Some(p) => p,
        None => return ERRNO_PERM,
    };
    trace_release(b"enter", pid);
    let _ = broker::release_for_device(pid, device_id);
    let _ = broker::irq_release_for_device(pid, device_id);
    let _ = broker::dma_release_for_device(pid, device_id);
    let _ = broker::pio_release_for_device(pid, device_id);
    match broker::release_device(pid, device_id) {
        Ok(_epoch) => {
            trace_release(b"ok", pid);
            0
        }
        Err(ClaimError::NotClaimed) => ERRNO_NODEV,
        Err(ClaimError::NotHolder) => ERRNO_PERM,
        Err(ClaimError::AlreadyClaimed) | Err(ClaimError::UnknownDevice) => ERRNO_INVAL,
    }
}
