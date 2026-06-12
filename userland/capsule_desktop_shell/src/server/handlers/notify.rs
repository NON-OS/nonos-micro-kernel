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

use nonos_libc::mk_time_millis;

use crate::compositor_client::push_damage_commit;
use crate::protocol::{Request, E_INVAL, NOTIFY_BODY_MAX, NOTIFY_REQ_LEN};
use crate::render::{menubar_rect, paint_chrome, sync_toast_layer};
use crate::server::respond;
use crate::state::{Context, NotifyLevel};

pub fn handle(ctx: &mut Context, sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    if body.len() != NOTIFY_REQ_LEN {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    }
    let Some(level_raw) = super::u32_at(body, 0) else {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    };
    let Some(body_len) = super::u32_at(body, 4) else {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    };
    let Some(level) = NotifyLevel::from_u32(level_raw) else {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    };
    if (body_len as usize) > NOTIFY_BODY_MAX {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    }
    ctx.last_notify_level = Some(level);
    let text_end = (8 + body_len as usize).min(body.len());
    ctx.toasts.push(&body[8..text_end], level, mk_time_millis());
    paint_chrome(ctx);
    let r = menubar_rect(ctx.width);
    let rid = ctx.issue_request_id();
    let _ = push_damage_commit(ctx.compositor_port, rid, r.x, r.y, r.width, r.height);
    sync_toast_layer(ctx);
    let _ = respond::status(sender_pid, req, 0, tx);
}
