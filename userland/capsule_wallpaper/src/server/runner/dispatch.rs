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

use crate::protocol::{
    Request, E_BAD_OP, E_INVAL, OP_FADE, OP_GET_WALLPAPER, OP_HEALTHCHECK, OP_SET_POLICY,
    OP_SET_WALLPAPER,
};
use crate::server::{handlers, respond};
use crate::state::Context;

pub fn dispatch(ctx: &mut Context, sender_pid: u32, req: Request, body: &[u8], tx: &mut [u8]) {
    match req.op {
        OP_HEALTHCHECK if body.is_empty() => handlers::health::handle(sender_pid, &req, tx),
        OP_SET_WALLPAPER => handlers::set_wallpaper::handle(ctx, sender_pid, &req, body, tx),
        OP_GET_WALLPAPER if body.is_empty() => handlers::get_wallpaper::handle(ctx, sender_pid, &req, tx),
        OP_SET_POLICY => handlers::set_policy::handle(ctx, sender_pid, &req, body, tx),
        OP_FADE => handlers::fade::handle(ctx, sender_pid, &req, body, tx),
        _ if body.is_empty() => {
            let _ = respond::status(sender_pid, &req, E_BAD_OP, tx);
        }
        _ => {
            let _ = respond::status(sender_pid, &req, E_INVAL, tx);
        }
    }
}
