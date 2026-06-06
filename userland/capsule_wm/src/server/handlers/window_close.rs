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

use crate::compositor_client::push_focus_set;
use crate::protocol::{Request, E_INVAL, E_NOENT, WINDOW_CLOSE_REQ_LEN};
use crate::server::{notify_fanout, respond};
use crate::state::Context;

pub fn handle(ctx: &mut Context, sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    if body.len() != WINDOW_CLOSE_REQ_LEN {
        if respond::status(sender_pid, req, E_INVAL, tx) < 0 {
            return;
        }
        return;
    }
    let Some(window_id) = super::u32_at::u32_at(body, 0) else {
        if respond::status(sender_pid, req, E_INVAL, tx) < 0 {
            return;
        }
        return;
    };
    if ctx.windows.find(sender_pid, window_id).is_none() {
        if respond::status(sender_pid, req, E_NOENT, tx) < 0 {
            return;
        }
        return;
    }
    if let Some(focused) = ctx.focus.current() {
        if focused.owner_pid == sender_pid && focused.window_id == window_id {
            let rid = ctx.issue_request_id();
            if push_focus_set(ctx.compositor_port, rid, 0).is_err() {
                if respond::status(sender_pid, req, E_INVAL, tx) < 0 {
                    return;
                }
                return;
            }
            if !ctx.focus.clear() {
                if respond::status(sender_pid, req, E_INVAL, tx) < 0 {
                    return;
                }
                return;
            }
        }
    }
    let Some(window) = ctx.windows.remove(sender_pid, window_id) else {
        if respond::status(sender_pid, req, E_NOENT, tx) < 0 {
            return;
        }
        return;
    };
    notify_fanout::broadcast(
        ctx,
        crate::protocol::NOTIFY_KIND_CLOSED,
        sender_pid,
        window_id,
        window.rect.x,
        window.rect.y,
    );
    if respond::status(sender_pid, req, 0, tx) < 0 {
        return;
    }
}
