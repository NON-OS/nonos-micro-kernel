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
use crate::protocol::{Request, E_INVAL, E_NOENT, WINDOW_MAXIMIZE_REQ_LEN};
use crate::server::respond;
use crate::state::Context;

pub fn handle(ctx: &mut Context, sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    if body.len() != WINDOW_MAXIMIZE_REQ_LEN {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    }
    let (Some(window_id), Some(x), Some(y), Some(w), Some(h)) = (
        super::u32_at::u32_at(body, 0),
        super::u32_at::u32_at(body, 8),
        super::u32_at::u32_at(body, 12),
        super::u32_at::u32_at(body, 16),
        super::u32_at::u32_at(body, 20),
    ) else {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    };
    if w == 0 || h == 0 {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    }
    let next =
        clamp_to_display(Rect { x, y, width: w, height: h }, ctx.display_width, ctx.display_height);
    let new_z = ctx.z.allocate();
    let Some(window) = ctx.windows.find_mut(sender_pid, window_id) else {
        let _ = respond::status(sender_pid, req, E_NOENT, tx);
        return;
    };
    window.rect = next;
    window.z = new_z;
    let _ = respond::status(sender_pid, req, 0, tx);
}
