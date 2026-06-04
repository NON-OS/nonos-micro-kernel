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

use crate::compositor_client::push_damage_commit;
use crate::decode_client::decode_and_paint;
use crate::paint::fill_argb;
use crate::protocol::{read_u32, Request, E_INVAL, SET_WALLPAPER_REQ_LEN};
use crate::server::respond;
use crate::state::Context;

pub fn handle(ctx: &mut Context, sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    if body.len() == SET_WALLPAPER_REQ_LEN {
        let Some(argb) = read_u32(body, 0) else {
            let _ = respond::status(sender_pid, req, E_INVAL, tx);
            return;
        };
        ctx.set_argb(argb);
        let composed = ctx.current_argb();
        fill_argb(ctx.backing_va, ctx.stride, ctx.width, ctx.height, composed);
    } else if decode_and_paint(ctx, body).is_err() {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    }
    let rid = ctx.issue_request_id();
    let _ = push_damage_commit(ctx.compositor_port, rid, 0, 0, ctx.width, ctx.height);
    let _ = respond::status(sender_pid, req, 0, tx);
}
