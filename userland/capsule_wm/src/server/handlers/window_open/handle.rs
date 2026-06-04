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

use crate::protocol::{Request, E_INVAL, E_NOMEM, NOTIFY_KIND_OPENED};
use crate::server::{notify_fanout, respond, respond_window_opened};
use crate::state::Context;
use crate::window::{Kind, Visibility, Window};

use super::decode::decode;
use super::focus_new_window::focus_new_window;
use super::place::place;

pub fn handle(ctx: &mut Context, sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    let Some((window_id, kind, requested)) = decode(body, ctx.display_width, ctx.display_height)
    else {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    };
    let rect = place(ctx, kind, requested);
    let z = ctx.z.allocate();
    let window = Window {
        owner_pid: sender_pid,
        window_id,
        rect,
        kind,
        visibility: Visibility::Visible,
        z,
        in_use: true,
    };
    if ctx.windows.insert(window).is_err() {
        let _ = respond::status(sender_pid, req, E_NOMEM, tx);
        return;
    }
    if kind == Kind::Normal {
        let _ = focus_new_window(ctx, sender_pid, window_id);
    }
    notify_fanout::broadcast(ctx, NOTIFY_KIND_OPENED, sender_pid, window_id, rect.x, rect.y);
    let _ = respond_window_opened::window_opened(sender_pid, req, 0, rect, tx);
}
