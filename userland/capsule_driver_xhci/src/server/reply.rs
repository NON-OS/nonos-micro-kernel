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

//! Reply routing for the request currently being served. The server
//! loop is strictly sequential: one `recv_from`, one dispatch, one
//! reply. So the sender pid captured before dispatch is the correct
//! destination for every send the handlers emit. A capsule caller
//! (pid != 0) gets a correlation-matched `mk_ipc_reply` delivered to
//! its own reply inbox; a kernel-internal caller (pid 0) keeps the
//! legacy fixed reply endpoint it drains by request id.

use core::sync::atomic::{AtomicU32, Ordering};

use nonos_libc::{mk_ipc_reply, mk_ipc_send};

use crate::protocol::KERNEL_REPLY_ENDPOINT;

static SENDER: AtomicU32 = AtomicU32::new(0);

pub fn set_sender(pid: u32) {
    SENDER.store(pid, Ordering::Relaxed);
}

pub fn send(tx: *const u8, len: usize) {
    let pid = SENDER.load(Ordering::Relaxed);
    if pid != 0 {
        let _ = mk_ipc_reply(pid, tx, len);
    } else {
        let _ = mk_ipc_send(KERNEL_REPLY_ENDPOINT, tx, len);
    }
}
