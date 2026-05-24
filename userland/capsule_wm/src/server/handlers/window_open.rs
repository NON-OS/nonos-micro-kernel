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
use crate::protocol::{Request, E_INVAL, E_NOMEM, NOTIFY_KIND_OPENED, WINDOW_OPEN_REQ_LEN};
use crate::server::{notify_fanout, respond};
use crate::state::Context;
use crate::window::{kind_from_u32, Visibility, Window};

pub fn handle(ctx: &mut Context, sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    let Some((window_id, kind, rect)) = decode(body, ctx.display_width, ctx.display_height) else {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    };
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
    notify_fanout::broadcast(ctx, NOTIFY_KIND_OPENED, sender_pid, window_id, rect.x, rect.y);
    let _ = respond::status(sender_pid, req, 0, tx);
}

fn decode(body: &[u8], display_w: u32, display_h: u32) -> Option<(u32, crate::window::Kind, Rect)> {
    if body.len() != WINDOW_OPEN_REQ_LEN {
        return None;
    }
    let window_id = super::u32_at(body, 0)?;
    let kind_raw = super::u32_at(body, 4)?;
    let x = super::u32_at(body, 8)?;
    let y = super::u32_at(body, 12)?;
    let w = super::u32_at(body, 16)?;
    let h = super::u32_at(body, 20)?;
    let kind = kind_from_u32(kind_raw)?;
    if window_id == 0 || w == 0 || h == 0 {
        return None;
    }
    Some((window_id, kind, clamp_to_display(Rect { x, y, width: w, height: h }, display_w, display_h)))
}
