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

use nonos_libc::mk_ipc_reply;

use crate::protocol::{write_header, HDR_LEN};

// Build and send a response. `sender_pid` came from
// `mk_ipc_recv_from`; the response lands in that pid's default
// inbox where the caller is blocked in its own `mk_ipc_recv` /
// `mk_ipc_call`. Buffer ownership: `tx` is laid out as 20-byte
// header followed by `payload_len` bytes the handler already
// wrote.
pub fn respond(
    sender_pid: u32,
    op: u16,
    errno: u16,
    request_id: u32,
    payload_len: u32,
    tx: &mut [u8],
) -> i64 {
    let total = write_header(tx, op, errno, request_id, payload_len);
    mk_ipc_reply(sender_pid, tx.as_ptr(), total)
}

pub fn respond_status_only(
    sender_pid: u32,
    op: u16,
    errno: u16,
    request_id: u32,
    tx: &mut [u8],
) -> i64 {
    debug_assert!(tx.len() >= HDR_LEN);
    respond(sender_pid, op, errno, request_id, 0, tx)
}
