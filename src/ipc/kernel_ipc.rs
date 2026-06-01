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

use core::sync::atomic::{AtomicU32, Ordering};

use crate::ipc::nonos_inbox::{self, StrictEnqueueError, KERNEL_OWNER};
use crate::process::caps;
use crate::services::registry::lookup_service;

static ROUTE_TRACE_COUNT: AtomicU32 = AtomicU32::new(0);

fn trace_route(caller_pid: u32, target: &str, dest_pid: u32, woke: bool) {
    if !matches!(dest_pid, 9 | 0x17) || ROUTE_TRACE_COUNT.fetch_add(1, Ordering::Relaxed) >= 40 {
        return;
    }
    crate::sys::serial::print(b"[ROUTE] from=");
    crate::sys::serial::print_hex(caller_pid as u64);
    crate::sys::serial::print(b" target=");
    crate::sys::serial::print(target.as_bytes());
    crate::sys::serial::print(b" dest=");
    crate::sys::serial::print_hex(dest_pid as u64);
    crate::sys::serial::print(if woke { b" wake=1" } else { b" wake=0" });
    crate::sys::serial::println(b"");
}

pub const EACCES: i32 = -13;
pub const ENOENT: i32 = -2;
pub const ESRCH: i32 = -3;
pub const ENOMEM: i32 = -12;
pub const EAGAIN: i32 = -11;

pub fn kernel_check_ipc_permission(caller_pid: u32, target: &str) -> Result<(), i32> {
    let endpoint = lookup_service(target).ok_or(ENOENT)?;
    if !caps::has(caller_pid, endpoint.caps_required) {
        return Err(EACCES);
    }
    Ok(())
}

pub fn kernel_route_ipc(caller_pid: u32, target: &str, data: &[u8]) -> Result<(), i32> {
    let endpoint = lookup_service(target).ok_or(ENOENT)?;
    if !caps::has(caller_pid, endpoint.caps_required) {
        return Err(EACCES);
    }
    // Capsule services route into the owning process inbox. Kernel-owned
    // reply endpoints route to the endpoint inbox the kernel client drains.
    let dest = if endpoint.pid == KERNEL_OWNER {
        alloc::string::String::from(target)
    } else {
        alloc::format!("proc.{}", endpoint.pid)
    };
    let msg = crate::ipc::nonos_channel::IpcMessage::new(
        &alloc::format!("proc.{}", caller_pid),
        &dest,
        data,
    )
    .map_err(|_| ENOMEM)?;
    match nonos_inbox::try_enqueue_strict(&dest, msg) {
        Ok(()) => {
            let woke = endpoint.pid != KERNEL_OWNER && crate::sched::is_sleeping(endpoint.pid);
            if woke {
                crate::sched::wake_process(endpoint.pid);
            }
            trace_route(caller_pid, target, endpoint.pid, woke);
            Ok(())
        }
        Err(StrictEnqueueError::MissingInbox) | Err(StrictEnqueueError::DeadOwner) => Err(ESRCH),
        Err(StrictEnqueueError::QueueFull(_)) => Err(EAGAIN),
    }
}
