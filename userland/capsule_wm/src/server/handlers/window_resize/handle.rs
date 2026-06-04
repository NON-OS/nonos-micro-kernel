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
use crate::geometry::{clamp_to_display, Rect};
use crate::protocol::{Request, E_INVAL, E_NOENT, WINDOW_RESIZE_REQ_LEN};
use crate::server::respond;
use crate::state::Context;
use crate::window::Kind;

use super::collides::collides;

pub fn handle(ctx: &mut Context, sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    if body.len() != WINDOW_RESIZE_REQ_LEN {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    }
    let Some(window_id) = crate::server::handlers::u32_at::u32_at(body, 0) else {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    };
    let Some(w) = crate::server::handlers::u32_at::u32_at(body, 8) else {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    };
    let Some(h) = crate::server::handlers::u32_at::u32_at(body, 12) else {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    };
    let Some(current) = ctx.windows.find(sender_pid, window_id) else {
        let _ = respond::status(sender_pid, req, E_NOENT, tx);
        return;
    };
    if w == 0 || h == 0 {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    }
    let r = Rect { x: current.rect.x, y: current.rect.y, width: w, height: h };
    let next = clamp_to_display(r, ctx.display_width, ctx.display_height);
    if current.kind == Kind::Normal && collides(ctx, sender_pid, window_id, next) {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    }
    let Some(window) = ctx.windows.find_mut(sender_pid, window_id) else {
        let _ = respond::status(sender_pid, req, E_NOENT, tx);
        return;
    };
    window.rect = next;
    let _ = respond::status(sender_pid, req, 0, tx);
}
