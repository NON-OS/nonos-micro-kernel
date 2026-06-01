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
    parse, E_BAD_OP, E_INVAL, HDR_LEN, IPC_PAYLOAD_MAX, OP_HEALTHCHECK, OP_NOTIFY,
    OP_SPOTLIGHT_OPEN, OP_TRAY_REGISTER, OP_TRAY_REMOVE, OP_TRAY_UPDATE,
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
        if super::input::handle(&mut ctx, &rx[..n as usize]) {
            continue;
        }
        let (req, body) = match parse(&rx[..n as usize]) {
            Ok(parsed) => parsed,
            Err((code, req)) => {
                let _ = respond::status(sender_pid, &req, code, &mut tx);
                continue;
            }
        };
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
        OP_TRAY_REGISTER => handlers::tray_register::handle(ctx, sender_pid, &req, body, tx),
        OP_TRAY_UPDATE => handlers::tray_update::handle(ctx, sender_pid, &req, body, tx),
        OP_TRAY_REMOVE => handlers::tray_remove::handle(ctx, sender_pid, &req, body, tx),
        OP_NOTIFY => handlers::notify::handle(ctx, sender_pid, &req, body, tx),
        OP_SPOTLIGHT_OPEN if body.is_empty() => {
            handlers::spotlight_open::handle(ctx, sender_pid, &req, tx)
        }
        _ if body.is_empty() => {
            let _ = respond::status(sender_pid, &req, E_BAD_OP, tx);
        }
        _ => {
            let _ = respond::status(sender_pid, &req, E_INVAL, tx);
        }
    }
}
