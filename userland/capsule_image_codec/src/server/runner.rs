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

use alloc::vec;

use nonos_libc::mk_ipc_recv_from;

use crate::protocol::{parse, E_BAD_OP, E_INVAL, HDR_LEN, IPC_PAYLOAD_MAX, OP_DECODE_BMP, OP_DECODE_JPEG, OP_DECODE_LZ4_RAW, OP_DECODE_PNG, OP_HEALTHCHECK};
use crate::server::{handlers, respond};

const SERVICE_INBOX: u64 = 0;
const RECV_BLOCK: u64 = 0;
const RECV_NOWAIT: u64 = 1;

pub fn run() -> ! {
    let mut rx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    let mut tx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    loop {
        drain_ipc(&mut rx, &mut tx);
    }
}

fn drain_ipc(rx: &mut [u8], tx: &mut [u8]) {
    let mut blocking = true;
    loop {
        let mut sender_pid = 0u32;
        let timeout = if blocking { RECV_BLOCK } else { RECV_NOWAIT };
        let n = mk_ipc_recv_from(SERVICE_INBOX, rx.as_mut_ptr(), rx.len(), timeout, &mut sender_pid);
        if n <= 0 || sender_pid == 0 {
            return;
        }
        blocking = false;
        let parsed = parse(&rx[..n as usize]);
        let (req, body) = match parsed {
            Ok(v) => v,
            Err((req, errno)) => { let _ = respond::status(sender_pid, &req, errno, tx); continue; }
        };
        match req.op {
            OP_HEALTHCHECK if body.is_empty() => handlers::health::handle(sender_pid, &req, tx),
            OP_DECODE_PNG | OP_DECODE_BMP | OP_DECODE_LZ4_RAW | OP_DECODE_JPEG => handlers::decode::handle(sender_pid, &req, body, tx),
            _ if body.is_empty() => { let _ = respond::status(sender_pid, &req, E_BAD_OP, tx); }
            _ => { let _ = respond::status(sender_pid, &req, E_INVAL, tx); }
        }
    }
}
