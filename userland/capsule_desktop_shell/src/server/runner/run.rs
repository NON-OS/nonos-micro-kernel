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

use nonos_libc::mk_time_millis;

use super::constants::CLOCK_REFRESH_MS;
use super::drain::drain;
use super::refresh_clock::refresh_clock;
use crate::protocol::{HDR_LEN, IPC_PAYLOAD_MAX};
use crate::render::sync_toast_layer;
use crate::server::refresh_taskbar::refresh_taskbar;
use crate::state::Context;

pub fn run(mut ctx: Context) -> ! {
    crate::server::paint_initial::paint_initial(&mut ctx);
    let mut rx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    let mut tx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    let mut last_clock_ms: i64 = 0;
    refresh_clock(&mut ctx);
    loop {
        drain(&mut ctx, &mut rx, &mut tx);
        let now = mk_time_millis();
        if now.wrapping_sub(last_clock_ms) >= CLOCK_REFRESH_MS as i64 {
            refresh_clock(&mut ctx);
            if !ctx.input_ready {
                let port = ctx.input_router_port;
                crate::setup::subscribe_input(&mut ctx, port);
            }
            if !ctx.wm_notify_ready {
                let port = ctx.wm_port;
                crate::setup::subscribe_wm(&mut ctx, port);
            }
            // Keep the desktop icons in step with the filesystem: this catches
            // the first listing once vfs_pool is up and any later change,
            // without ever wiping a good desktop on a transient empty reply.
            if crate::server::desktop::refresh(&mut ctx) {
                crate::server::repaint::repaint(&mut ctx);
            }
            crate::server::installed_apps::load_once(&mut ctx);
            crate::server::packages::refresh(&mut ctx);
            crate::server::store_health::check(&mut ctx);
            ctx.toasts.expire(now);
            if !ctx.toasts.is_empty() || ctx.toast_layer_live {
                sync_toast_layer(&mut ctx);
            }
            let pulse_dirty = crate::state::expire_taskbar_pulses(&mut ctx.taskbar, now);
            let reveal_dirty = crate::state::expire_taskbar_visibility(&mut ctx.taskbar, now);
            if pulse_dirty || reveal_dirty {
                refresh_taskbar(&mut ctx);
            }
            last_clock_ms = now;
        }
        // No vsync sleep: drain() already blocks on the IPC inbox (waking the
        // instant an input or notify message arrives, and timing out every
        // RECV_BLOCK ms so the clock still ticks). Parking again on the vsync
        // tick only added up to a frame of latency to every click, since an
        // inbound event cannot preempt a vsync wait.
    }
}
