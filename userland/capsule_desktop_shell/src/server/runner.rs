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

use nonos_libc::{mk_display_vsync_wait, mk_ipc_recv_from};

use crate::compositor_client::push_damage_commit;
use crate::protocol::{
    parse, E_BAD_OP, E_INVAL, HDR_LEN, IPC_PAYLOAD_MAX, OP_HEALTHCHECK, OP_NOTIFY,
    OP_SPOTLIGHT_OPEN, OP_TRAY_REGISTER, OP_TRAY_REMOVE, OP_TRAY_UPDATE,
};
use crate::render::{menubar_rect, paint_chrome, paint_status};
use crate::server::{handlers, respond};
use crate::state::Context;

const SERVICE_INBOX: u64 = 0;
const RECV_NOWAIT: u64 = 1;
const CLOCK_TICKS: u32 = 60;

pub fn run(mut ctx: Context) -> ! {
    let mut rx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    let mut tx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    let mut ticks: u32 = 0;
    refresh_clock(&mut ctx);
    loop {
        drain(&mut ctx, &mut rx, &mut tx);
        ticks = ticks.wrapping_add(1);
        if ticks % CLOCK_TICKS == 0 {
            refresh_clock(&mut ctx);
        }
        let _ = mk_display_vsync_wait(0);
    }
}

fn drain(ctx: &mut Context, rx: &mut [u8], tx: &mut [u8]) {
    loop {
        let mut sender_pid = 0u32;
        let n = mk_ipc_recv_from(SERVICE_INBOX, rx.as_mut_ptr(), rx.len(), RECV_NOWAIT, &mut sender_pid);
        if n <= 0 || sender_pid == 0 {
            return;
        }
        let (req, body) = match parse(&rx[..n as usize]) {
            Ok(parsed) => parsed,
            Err((code, req)) => {
                let _ = respond::status(sender_pid, &req, code, tx);
                continue;
            }
        };
        dispatch(ctx, sender_pid, req, body, tx);
    }
}

fn refresh_clock(ctx: &mut Context) {
    paint_chrome(ctx);
    paint_status(ctx);
    let r = menubar_rect(ctx.width);
    let rid = ctx.issue_request_id();
    let _ = push_damage_commit(ctx.compositor_port, rid, r.x, r.y, r.width, r.height);
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
