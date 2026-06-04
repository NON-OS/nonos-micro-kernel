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

use nonos_libc::mk_ipc_recv_from;

use crate::protocol::{
    parse, E_BAD_OP, E_INVAL, OP_GRAB_RELEASE, OP_GRAB_REQUEST, OP_HEALTHCHECK, OP_SUBSCRIBE,
};
use crate::server::{handlers, respond};
use crate::state::Context;

const SERVICE_INBOX: u64 = 0;
const RECV_NOWAIT: u64 = 1;

pub(super) fn drain_ipc(ctx: &mut Context, rx: &mut [u8], tx: &mut [u8]) {
    loop {
        let mut sender_pid = 0u32;
        let n = mk_ipc_recv_from(
            SERVICE_INBOX,
            rx.as_mut_ptr(),
            rx.len(),
            RECV_NOWAIT,
            &mut sender_pid,
        );
        if n <= 0 || sender_pid == 0 {
            return;
        }
        let Some((req, body)) = parse(&rx[..n as usize]) else {
            continue;
        };
        match req.op {
            OP_HEALTHCHECK if body.is_empty() => handlers::health::handle(sender_pid, &req, tx),
            OP_SUBSCRIBE => handlers::subscribe::handle(ctx, sender_pid, &req, body, tx),
            OP_GRAB_REQUEST => handlers::grab_request::handle(ctx, sender_pid, &req, body, tx),
            OP_GRAB_RELEASE if body.is_empty() => {
                handlers::grab_release::handle(ctx, sender_pid, &req, tx)
            }
            _ if body.is_empty() => {
                let _ = respond::status(sender_pid, &req, E_BAD_OP, tx);
            }
            _ => {
                let _ = respond::status(sender_pid, &req, E_INVAL, tx);
            }
        }
    }
}
