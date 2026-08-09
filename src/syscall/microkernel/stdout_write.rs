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

//! `MkStdoutWrite` handler. Program stdout for a capsule that holds no
//! `Capability::Debug` token: the bytes are mirrored into the caller's
//! own `proc.<pid>` inbox for its launcher to drain, and nothing is
//! written to the boot serial or the framebuffer console.
//!
//! The contract layer has already verified `Capability::IPC`; this layer
//! only validates the user buffer. Bounded to `MAX_LEN` bytes after which
//! the syscall returns `-EINVAL`. Empty calls are also rejected. Bytes
//! are passed through verbatim — this is a program's own output, not a
//! trust log, so it is never rewritten.

use super::errnos::{ERRNO_FAULT, ERRNO_INVAL};

const MAX_LEN: usize = 256;

pub fn sys_stdout_write(user_ptr: u64, len: u64) -> i64 {
    if user_ptr == 0 || len == 0 {
        return ERRNO_INVAL;
    }
    let len = len as usize;
    if len > MAX_LEN {
        return ERRNO_INVAL;
    }
    match crate::usercopy::validate_user_read(user_ptr, len) {
        Ok(()) => {}
        Err(_) => return ERRNO_FAULT,
    }
    let mut buf = [0u8; MAX_LEN];
    if crate::usercopy::copy_from_user(user_ptr, &mut buf[..len]).is_err() {
        return ERRNO_FAULT;
    }
    if crate::syscall::caps::current_caps_or_default().can_debug() {
        crate::sys::serial::print(&buf[..len]);
    }
    super::debug::mirror_to_proc_inbox(&buf[..len]);
    len as i64
}
