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
use crate::protocol::{parse, HDR_LEN, IPC_PAYLOAD_MAX};
use crate::render::{menubar_rect, paint_chrome, paint_status};
use crate::server::respond;
use crate::state::Context;

const SERVICE_INBOX: u64 = 0;
const RECV_BLOCK: u64 = 0;
const RECV_RETRY_MS: u64 = 16;
const CLOCK_TICKS: u32 = 60;

pub fn run(mut ctx: Context) -> ! {
    super::paint_initial::paint_initial(&mut ctx);
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
        let timeout = if super::ready_to_block::ready_to_block(ctx) {
            RECV_BLOCK
        } else {
            RECV_RETRY_MS
        };
        let n = mk_ipc_recv_from(SERVICE_INBOX, rx.as_mut_ptr(), rx.len(), timeout, &mut sender_pid);
        super::retry_input_subscription::retry_input_subscription(ctx);
        super::retry_wm_subscription::retry_wm_subscription(ctx);
        if n <= 0 || sender_pid == 0 {
            return;
        }
        if super::wm_notify::handle(ctx, &rx[..n as usize]) {
            continue;
        }
        if super::input::handle(ctx, &rx[..n as usize]) {
            continue;
        }
        let (req, body) = match parse(&rx[..n as usize]) {
            Ok(parsed) => parsed,
            Err((code, req)) => {
                let _ = respond::status(sender_pid, &req, code, tx);
                continue;
            }
        };
        super::dispatch::dispatch(ctx, sender_pid, req, body, tx);
    }
}

fn refresh_clock(ctx: &mut Context) {
    paint_chrome(ctx);
    paint_status(ctx);
    let r = menubar_rect(ctx.width);
    let rid = ctx.issue_request_id();
    let _ = push_damage_commit(ctx.compositor_port, rid, r.x, r.y, r.width, r.height);
}
