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
    parse, E_BAD_OP, E_INVAL, HDR_LEN, IPC_PAYLOAD_MAX, OP_HEALTHCHECK, OP_LIFECYCLE_SUBSCRIBE,
    OP_QUERY_TOPMOST, OP_ROUTE_FOCUS, OP_WINDOW_CLOSE, OP_WINDOW_FOCUS, OP_WINDOW_MINIMIZE,
    OP_WINDOW_MOVE, OP_WINDOW_OPEN, OP_WINDOW_RAISE, OP_WINDOW_RESIZE, OP_WINDOW_RESTORE,
};
use crate::server::{handlers, respond};
use crate::state::Context;

const SERVICE_INBOX: u64 = 0;

pub fn run(mut ctx: Context) -> ! {
    let mut rx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    let mut tx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    loop {
        let mut sender_pid = 0u32;
        let n = mk_ipc_recv_from(SERVICE_INBOX, rx.as_mut_ptr(), rx.len(), 0, &mut sender_pid);
        if n <= 0 || sender_pid == 0 {
            continue;
        }
        let Some((req, body)) = parse(&rx[..n as usize]) else { continue };
        dispatch(&mut ctx, sender_pid, req, body, &mut tx);
    }
}

fn dispatch(
    ctx: &mut Context,
    sender_pid: u32,
    req: crate::protocol::Request,
    body: &[u8],
    tx: &mut [u8],
) {
    match req.op {
        OP_HEALTHCHECK if body.is_empty() => handlers::health::handle(sender_pid, &req, tx),
        OP_WINDOW_OPEN => handlers::window_open::handle(ctx, sender_pid, &req, body, tx),
        OP_WINDOW_CLOSE => handlers::window_close::handle(ctx, sender_pid, &req, body, tx),
        OP_WINDOW_MOVE => handlers::window_move::handle(ctx, sender_pid, &req, body, tx),
        OP_WINDOW_RESIZE => handlers::window_resize::handle(ctx, sender_pid, &req, body, tx),
        OP_WINDOW_FOCUS => handlers::window_focus::handle(ctx, sender_pid, &req, body, tx),
        OP_WINDOW_RAISE => handlers::window_raise::handle(ctx, sender_pid, &req, body, tx),
        OP_WINDOW_MINIMIZE => handlers::window_minimize::handle(ctx, sender_pid, &req, body, tx),
        OP_WINDOW_RESTORE => handlers::window_restore::handle(ctx, sender_pid, &req, body, tx),
        OP_QUERY_TOPMOST => handlers::query_topmost::handle(ctx, sender_pid, &req, body, tx),
        OP_ROUTE_FOCUS => handlers::route_focus::handle(ctx, sender_pid, &req, body, tx),
        OP_LIFECYCLE_SUBSCRIBE if body.is_empty() => {
            handlers::lifecycle_subscribe::handle(ctx, sender_pid, &req, tx)
        }
        _ if body.is_empty() => {
            let _ = respond::status(sender_pid, &req, E_BAD_OP, tx);
        }
        _ => {
            let _ = respond::status(sender_pid, &req, E_INVAL, tx);
        }
    }
}
