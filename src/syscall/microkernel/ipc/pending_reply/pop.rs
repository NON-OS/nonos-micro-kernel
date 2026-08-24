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

use alloc::string::String;

use super::state::PENDING;

// A server replying with a token that matches no pending caller: the reply
// stream desynced from the request stream. Distinct from the token-0 kernel
// case, which is expected and silent.
static DESYNC: crate::sys::diag::Site = crate::sys::diag::Site::new(b"ipc.reply");

pub(in crate::syscall::microkernel::ipc) fn pop(server_pid: u32) -> Option<(u32, String, u64)> {
    // Pop the entry whose token matches the request this server dequeued
    // last, never the positional front. Kernel-side senders push no entry but
    // their requests still occupy the server's inbox, so front-popping paired
    // some capsule caller's reply with whatever request happened to precede
    // it. A server answering a kernel request finds no match here, and the
    // send path drops that reply as addressed to nobody, which is what it is.
    let want = super::state::last_served(server_pid);
    if want == 0 {
        return None;
    }
    let mut map = PENDING.lock();
    let queue = map.get_mut(&server_pid)?;
    let Some(pos) = queue.iter().position(|(_, _, token)| *token == want) else {
        // The server replied stamped with a request token that no caller is
        // waiting on. That is a genuine correlation desync, not the benign
        // kernel-request case (which carries token 0 and returns above): the
        // reply is about to be sent as addressed to a reply endpoint nobody is
        // draining, and its caller will time out. Loud, because it names the
        // exact server whose reply stream slipped.
        DESYNC.note("reply token had no waiting caller", server_pid as u64);
        return None;
    };
    let entry = queue.remove(pos);
    if queue.is_empty() {
        map.remove(&server_pid);
    }
    // One reply per request: a second reply to the same request finds nothing.
    super::state::record_served(server_pid, 0);
    entry
}
