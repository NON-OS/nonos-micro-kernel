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
    Request, E_BAD_OP, E_INVAL, OP_CURSOR_UPDATE, OP_DAMAGE_COMMIT, OP_DISPLAY_INFO, OP_FOCUS_SET,
    OP_HEALTHCHECK, OP_INPUT_SUBSCRIBE, OP_SCENE_REMOVE, OP_SCENE_SUBMIT,
};
use crate::server::{handlers, respond};
use crate::state::Context;

pub fn dispatch(
    ctx: &mut Context,
    sender_pid: u32,
    req: Request,
    body: &[u8],
    tx: &mut [u8],
) -> Result<(), &'static str> {
    match req.op {
        OP_HEALTHCHECK if body.is_empty() => handlers::health::handle(sender_pid, &req, tx),
        OP_SCENE_SUBMIT => handlers::scene_submit::handle(ctx, sender_pid, &req, body, tx),
        OP_SCENE_REMOVE => handlers::scene_remove::handle(ctx, sender_pid, &req, body, tx),
        OP_DAMAGE_COMMIT => handlers::damage_commit::handle(ctx, sender_pid, &req, body, tx),
        OP_FOCUS_SET => handlers::focus_set::handle(ctx, sender_pid, &req, body, tx),
        OP_CURSOR_UPDATE => handlers::cursor_update::handle(ctx, sender_pid, &req, body, tx),
        OP_INPUT_SUBSCRIBE if body.is_empty() => {
            handlers::input_subscribe::handle(ctx, sender_pid, &req, tx)
        }
        OP_DISPLAY_INFO if body.is_empty() => {
            handlers::display_info::handle(ctx, sender_pid, &req, tx)
        }
        _ if body.is_empty() => respond::status(sender_pid, &req, E_BAD_OP, tx),
        _ => respond::status(sender_pid, &req, E_INVAL, tx),
    }
}
