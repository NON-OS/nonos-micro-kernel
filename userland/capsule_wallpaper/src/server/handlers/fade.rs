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

use nonos_libc::mk_display_vsync_wait;

use crate::compositor_client::push_damage_commit;
use crate::paint::fill_argb;
use crate::protocol::{read_u32, Request, E_INVAL, FADE_REQ_LEN};
use crate::server::respond;
use crate::state::Context;

pub fn handle(ctx: &mut Context, sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    if body.len() != FADE_REQ_LEN {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    }
    let Some(target_alpha) = read_u32(body, 0) else {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    };
    let Some(duration_ms) = read_u32(body, 4) else {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    };
    if target_alpha > 0xFF {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    }
    let now = mk_display_vsync_wait(0);
    let now_ns = if now > 0 { now as u64 } else { 0 };
    let duration_ns = (duration_ms as u64).saturating_mul(1_000_000);
    ctx.fade.start(ctx.alpha, target_alpha as u8, now_ns, duration_ns);
    if duration_ns == 0 {
        ctx.alpha = target_alpha as u8;
        let argb = ctx.current_argb();
        fill_argb(ctx.backing_va, ctx.stride, ctx.width, ctx.height, argb);
        let rid = ctx.issue_request_id();
        let _ = push_damage_commit(ctx.compositor_port, rid, 0, 0, ctx.width, ctx.height);
    }
    let _ = respond::status(sender_pid, req, 0, tx);
}
