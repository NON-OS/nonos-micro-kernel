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

//! `MkDebug` handler. A capsule emits one short diagnostic line on the
//! boot serial. The contract layer has already verified the
//! `Capability::Debug` token; this layer only validates the user
//! buffer and writes it through.
//!
//! The line is bounded to `MAX_LEN` bytes after which the syscall
//! returns `-EINVAL`. Empty calls are also rejected. Non-printable
//! bytes are passed through verbatim — the harness greps for exact
//! marker strings, and silently rewriting them would defeat the
//! purpose of having the channel.

use super::errnos::{ERRNO_FAULT, ERRNO_INVAL};

const MAX_LEN: usize = 256;

pub fn sys_mk_debug(user_ptr: u64, len: u64) -> i64 {
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
    crate::sys::serial::print(&buf[..len]);
    // Mirror to the on-screen log too: a capsule reporting its bring-up on a
    // machine with no serial port is otherwise invisible. No-op unless the
    // framebuffer console is enabled (NONOS_FBCONSOLE=1 bring-up build).
    //
    // Silenced for a clean desktop: capsule diagnostics now surface in the
    // Settings panel and the in-system log viewer, so they no longer scroll over
    // the framebuffer. Uncomment to bring the on-screen capsule trace back for a
    // headless bring-up.
    /* crate::sys::boot_log::capsule_screen(&buf[..len]); */
    mirror_to_proc_inbox(&buf[..len]);
    len as i64
}

// Mirror the line into the calling process's own `proc.<pid>` inbox so a
// launcher (the terminal) can drain a child capsule's stdout into its
// window. Best effort: a missing or full inbox is ignored, and serial
// above stays the source of truth for trust logs.
pub(super) fn mirror_to_proc_inbox(bytes: &[u8]) {
    let Some(pid) = crate::process::current_pid() else {
        return;
    };
    let name = alloc::format!("proc.{}", pid);
    // Skip the copy when the inbox is missing or already full. Nothing is
    // draining most capsules, so their inbox fills once and then every later
    // line is dropped here without building a message.
    if !crate::ipc::nonos_inbox::exists(&name) || crate::ipc::nonos_inbox::is_full(&name) {
        return;
    }
    if let Ok(msg) = crate::ipc::nonos_channel::IpcMessage::new(&name, &name, bytes) {
        let _ = crate::ipc::nonos_inbox::try_enqueue_strict(&name, msg);
    }
}
