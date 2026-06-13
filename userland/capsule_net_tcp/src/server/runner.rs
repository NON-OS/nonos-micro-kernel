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

use crate::protocol::{
    E_BAD_OP, IPC_PAYLOAD_MAX, OP_ACCEPT, OP_CLOSE, OP_CONNECT, OP_HEALTHCHECK, OP_LISTEN, OP_RECV,
    OP_SEND, OP_SHUTDOWN, OP_STATE,
};

use super::handlers;
use super::parse_req::{parse, HDR_LEN};
use super::respond::respond;
use super::tick::{recv_budget, tick};

const SERVICE_INBOX: u64 = 0;

pub fn run() -> ! {
    let mut rx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    let mut tx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    loop {
        tick();
        let budget = recv_budget();
        let mut sender_pid = 0u32;
        let n = mk_ipc_recv_from(SERVICE_INBOX, rx.as_mut_ptr(), rx.len(), budget, &mut sender_pid);
        if n <= 0 || sender_pid == 0 {
            continue;
        }
        let Ok((req, body)) = parse(&rx[..n as usize]) else { continue };
        match req.op {
            OP_HEALTHCHECK => handlers::health::handle(sender_pid, &req, &mut tx),
            OP_LISTEN => handlers::listen::handle(sender_pid, &req, body, &mut tx),
            OP_CONNECT => handlers::connect::handle(sender_pid, &req, body, &mut tx),
            OP_ACCEPT => handlers::accept::handle(sender_pid, &req, body, &mut tx),
            OP_SEND => handlers::send::handle(sender_pid, &req, body, &mut tx),
            OP_RECV => handlers::recv::handle(sender_pid, &req, body, &mut tx),
            OP_CLOSE | OP_SHUTDOWN => handlers::close::handle(sender_pid, &req, body, &mut tx),
            OP_STATE => handlers::state::handle(sender_pid, &req, body, &mut tx),
            _ => {
                let _ = respond(sender_pid, req.op, E_BAD_OP, req.request_id, 0, &mut tx);
            }
        }
    }
}
