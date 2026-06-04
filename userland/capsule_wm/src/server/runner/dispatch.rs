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
    Request, E_BAD_OP, E_INVAL, OP_HEALTHCHECK, OP_LIFECYCLE_SUBSCRIBE, OP_QUERY_FOCUS,
    OP_QUERY_TOPMOST, OP_ROUTE_FOCUS, OP_WINDOW_CLOSE, OP_WINDOW_FOCUS, OP_WINDOW_MINIMIZE,
    OP_WINDOW_MOVE, OP_WINDOW_OPEN, OP_WINDOW_RAISE, OP_WINDOW_RESIZE, OP_WINDOW_RESTORE,
};
use crate::server::{handlers, respond};
use crate::state::Context;

pub(super) fn dispatch(
    ctx: &mut Context,
    sender_pid: u32,
    req: Request,
    body: &[u8],
    tx: &mut [u8],
) {
    match req.op {
        OP_HEALTHCHECK if body.is_empty() => handlers::health::handle(sender_pid, &req, tx),
        OP_WINDOW_OPEN => handlers::window_open::handle(ctx, sender_pid, &req, body, tx),
        OP_WINDOW_CLOSE => handlers::window_close::handle(ctx, sender_pid, &req, body, tx),
        OP_WINDOW_MOVE => handlers::window_move::handle(ctx, sender_pid, &req, body, tx),
        OP_WINDOW_RESIZE => handlers::window_resize::handle(ctx, sender_pid, &req, body, tx),
        OP_WINDOW_FOCUS => handlers::window_focus::handle(ctx, sender_pid, &req, body, tx),
        OP_WINDOW_RAISE => handlers::window_raise::handle(ctx, sender_pid, &req, body, tx),
        OP_WINDOW_MINIMIZE => handlers::window_minimize::handle(ctx, sender_pid, &req, body, tx),
        OP_WINDOW_RESTORE => handlers::window_restore::handle(ctx, sender_pid, &req, body, tx),
        OP_QUERY_TOPMOST => handlers::query_topmost::handle(ctx, sender_pid, &req, body, tx),
        OP_QUERY_FOCUS if body.is_empty() => {
            handlers::query_focus::handle(ctx, sender_pid, &req, tx)
        }
        OP_ROUTE_FOCUS => handlers::route_focus::handle(ctx, sender_pid, &req, body, tx),
        OP_LIFECYCLE_SUBSCRIBE if body.is_empty() => {
            handlers::lifecycle_subscribe::handle(ctx, sender_pid, &req, tx)
        }
        _ if body.is_empty() => {
            let _ = respond::status(sender_pid, &req, E_BAD_OP, tx);
        }
        _ => {
            let _ = respond::status(sender_pid, &req, E_INVAL, tx);
        }
    }
}
