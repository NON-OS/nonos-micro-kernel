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

extern crate alloc;

use alloc::vec;

use crate::capabilities::Capability;
use crate::process::current_pid;
use crate::services::registry::{register_endpoint, required_caps, RegError};
use crate::syscall::microkernel::errnos::{
    ERRNO_BUSY, ERRNO_FAULT, ERRNO_INVAL, ERRNO_NOMEM, ERRNO_PERM,
};

const NAME_MAX: usize = 64;

// `MkServiceRegister(name_ptr, name_len, port)`. Anchors the calling
// capsule as the owner of the named service on the given port so
// peers can resolve it via `MkServiceLookup` without hardcoding the
// wire-side endpoint. The kernel records (name, port, pid) under
// the caller's pid; teardown sweeps the entry when the capsule exits.
pub fn sys_service_register(name_ptr: u64, name_len: usize, port: u32) -> i64 {
    if name_len == 0 || name_len > NAME_MAX || port == 0 {
        return ERRNO_INVAL;
    }
    if crate::usercopy::validate_user_read(name_ptr, name_len).is_err() {
        return ERRNO_FAULT;
    }
    let pid = match current_pid() {
        Some(p) => p,
        None => return ERRNO_PERM,
    };
    let mut name_buf = vec![0u8; name_len];
    if crate::usercopy::copy_from_user(name_ptr, &mut name_buf).is_err() {
        return ERRNO_FAULT;
    }
    let name = match core::str::from_utf8(&name_buf) {
        Ok(s) => s,
        Err(_) => return ERRNO_INVAL,
    };
    if name.starts_with("proc.") || name.starts_with("endpoint.") {
        return ERRNO_PERM;
    }
    // Core service names/ports are owned by the kernel spawn path; a capsule
    // must never register them at runtime. This closes the post-crash squat
    // window where a capsule could impersonate keyring/crypto/vfs. Legitimate
    // (re)spawn registration bypasses this syscall, so it is unaffected.
    if crate::services::registry::is_reserved_service(name, port) {
        return ERRNO_PERM;
    }
    match register_endpoint(name, port, pid, required_caps(name, Capability::IPC.bit())) {
        Ok(()) => 0,
        Err(RegError::Full) => ERRNO_NOMEM,
        Err(RegError::Exists) => ERRNO_BUSY,
        Err(RegError::PermissionDenied) => ERRNO_PERM,
        Err(RegError::NotFound) => ERRNO_INVAL,
    }
}
