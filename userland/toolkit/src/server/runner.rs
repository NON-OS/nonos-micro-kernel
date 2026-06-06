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

use nonos_libc::{mk_exit, mk_ipc_recv_from, mk_ipc_reply};

use crate::protocol::{decode, encode, Header, HDR_LEN, IPC_PAYLOAD_MAX, TOOLKIT_ENDPOINT};

use super::dispatch;

const ENOTSUP: i64 = -95;

pub fn run() -> ! {
    let mut rx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    let mut tx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    loop {
        let mut sender_pid = 0u32;
        let n = mk_ipc_recv_from(TOOLKIT_ENDPOINT, rx.as_mut_ptr(), rx.len(), 0, &mut sender_pid);
        if n == ENOTSUP {
            mk_exit(95);
        }
        if n <= 0 || sender_pid == 0 {
            continue;
        }
        let used = n as usize;
        let Some(hdr) = decode(&rx[..used]) else {
            continue;
        };
        let payload = &rx[HDR_LEN..used];
        let (status, reply_len) = dispatch::dispatch(hdr.op, payload, &mut tx[HDR_LEN..]);
        let reply_hdr =
            Header { op: hdr.op, request_id: hdr.request_id, payload_len: reply_len as u32 };
        encode(&mut tx[..HDR_LEN], &reply_hdr, status);
        if mk_ipc_reply(sender_pid, tx.as_ptr(), HDR_LEN + reply_len) < 0 {
            continue;
        }
    }
}
