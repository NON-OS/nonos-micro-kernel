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

use nonos_libc::{mk_input_event_wait, InputEvent};

use crate::clients::compositor;
use crate::protocol::{HDR_LEN, IPC_PAYLOAD_MAX};
use crate::route::route_event;
use crate::sources::{drain_batch, kernel_ring::MAX_BATCH};
use crate::state::Context;

use super::drain_ipc::drain_ipc;

// Upper bound on how long the router parks when the kernel input ring is empty.
// A posted event wakes it immediately; this only bounds the case where the wake
// signal is missed, so keeping it well under a frame stops a click from waiting
// on the timeout before it is routed to the focused window.
const INPUT_WAIT_MS: u64 = 8;

pub fn run() -> ! {
    let mut ctx = Context::new();
    let mut rx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    let mut tx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    let mut batch: [InputEvent; MAX_BATCH] = [InputEvent::default(); MAX_BATCH];
    let mut last_seq: u64 = 0;
    let mut sweep_ticks: u32 = 0;
    loop {
        drain_ipc(&mut ctx, &mut rx, &mut tx);
        sweep_ticks = sweep_ticks.wrapping_add(1);
        if sweep_ticks & 0x3f == 0 {
            ctx.purge_dead();
            let now_ms = nonos_libc::mk_time_millis();
            if now_ms.wrapping_sub(ctx.last_policy_ms) >= 2000 {
                ctx.last_policy_ms = now_ms;
                if let Some(v) = crate::clients::policy::mouse_sensitivity(&mut ctx.policy_port) {
                    ctx.cursor.mult_x2 = v.clamp(1, 4) as i32;
                }
            }
        }
        let n = drain_batch(&mut batch);
        for ev in batch.iter().take(n) {
            route_event(&mut ctx, ev);
        }
        if ctx.cursor_dirty {
            let rid = ctx.issue_request_id();
            let _ = compositor::cursor_update(
                &mut ctx.compositor_port,
                rid,
                ctx.cursor_x,
                ctx.cursor_y,
                true,
            );
            ctx.cursor_dirty = false;
        }
        if n == 0 {
            let mut seq = last_seq;
            let _ = mk_input_event_wait(last_seq, INPUT_WAIT_MS, &mut seq);
            last_seq = seq;
        }
    }
}
