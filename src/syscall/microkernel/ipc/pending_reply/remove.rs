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

use super::state::PENDING;

/// Remove the pending-reply entry for `caller_inbox` under `server_pid` and
/// return the per-call correlation token it carried. Matching is by the
/// caller's private inbox, so the token is the one that exact `mk_ipc_call`
/// registered, even when several callers have replies outstanding on the same
/// server. Returns `None` when there is no such pending call (a forged or
/// duplicate reply), which the caller treats as "drop".
pub(in crate::syscall::microkernel::ipc) fn remove(
    server_pid: u32,
    caller_inbox: &str,
) -> Option<u64> {
    let mut map = PENDING.lock();
    let queue = map.get_mut(&server_pid)?;
    let pos = queue.iter().position(|(_, inbox, _)| inbox == caller_inbox)?;
    // main's entry is (caller_pid, inbox, token); return the token so the direct
    // mk_ipc_reply path stamps the reply from the same entry it removes.
    let (_, _, token) = queue.remove(pos)?;
    if queue.is_empty() {
        map.remove(&server_pid);
    }
    Some(token)
}

/// Remove the newest entry for `caller_inbox`. The failed-send path must strip
/// the entry it just pushed and only that one: this caller may still be owed an
/// older reply from an earlier timed-out call, and taking the older entry shifts
/// the server's whole FIFO onto the wrong callers.
pub(in crate::syscall::microkernel::ipc) fn remove_latest(server_pid: u32, caller_inbox: &str) {
    let mut map = PENDING.lock();
    let Some(queue) = map.get_mut(&server_pid) else { return };
    if let Some(pos) = queue.iter().rposition(|(_, inbox, _)| inbox == caller_inbox) {
        queue.remove(pos);
    }
    if queue.is_empty() {
        map.remove(&server_pid);
    }
}
