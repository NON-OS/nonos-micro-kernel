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
use crate::protocol::{Request, E_INVAL, E_NOENT, E_PERM, ROUTE_FOCUS_REQ_LEN};
use crate::server::respond;
use crate::state::Context;

use super::is_input_router::is_input_router;

pub fn handle(ctx: &mut Context, sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    if body.len() != ROUTE_FOCUS_REQ_LEN || !is_input_router(sender_pid) {
        if respond::status(sender_pid, req, E_PERM, tx) < 0 {
            return;
        }
        return;
    }
    let Some(owner_pid) = super::super::u32_at::u32_at(body, 0) else {
        if respond::status(sender_pid, req, E_INVAL, tx) < 0 {
            return;
        }
        return;
    };
    let Some(window_id) = super::super::u32_at::u32_at(body, 4) else {
        if respond::status(sender_pid, req, E_INVAL, tx) < 0 {
            return;
        }
        return;
    };
    let Some(window) = ctx.windows.find(owner_pid, window_id) else {
        if respond::status(sender_pid, req, E_NOENT, tx) < 0 {
            return;
        }
        return;
    };
    if !window.kind.focusable() {
        if respond::status(sender_pid, req, E_PERM, tx) < 0 {
            return;
        }
        return;
    }
    if !matches!(ctx.focus.current(), Some(f) if f.owner_pid == owner_pid && f.window_id == window_id)
    {
        if !ctx.focus.set(owner_pid, window_id) {
            if respond::status(sender_pid, req, E_INVAL, tx) < 0 {
                return;
            }
            return;
        }
        let rid = ctx.issue_request_id();
        let _ = push_focus_set(ctx.compositor_port, rid, owner_pid);
    }
    if respond::status(sender_pid, req, 0, tx) < 0 {
        return;
    }
}
