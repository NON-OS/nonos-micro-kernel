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
    Request, E_BAD_OP, E_INVAL, OP_HEALTHCHECK, OP_NOTIFY, OP_OPEN_WITH, OP_SPOTLIGHT_OPEN,
    OP_TAKE_OPEN_ARG, OP_TRAY_REGISTER, OP_TRAY_REMOVE, OP_TRAY_UPDATE,
};
use crate::server::{handlers, respond};
use crate::state::Context;

pub fn dispatch(ctx: &mut Context, sender_pid: u32, req: Request, body: &[u8], tx: &mut [u8]) {
    match req.op {
        OP_HEALTHCHECK if body.is_empty() => handlers::health::handle(sender_pid, &req, tx),
        OP_TRAY_REGISTER => handlers::tray_register::handle(ctx, sender_pid, &req, body, tx),
        OP_TRAY_UPDATE => handlers::tray_update::handle(ctx, sender_pid, &req, body, tx),
        OP_TRAY_REMOVE => handlers::tray_remove::handle(ctx, sender_pid, &req, body, tx),
        OP_NOTIFY => handlers::notify::handle(ctx, sender_pid, &req, body, tx),
        OP_SPOTLIGHT_OPEN if body.is_empty() => {
            handlers::spotlight_open::handle(ctx, sender_pid, &req, tx)
        }
        OP_OPEN_WITH => handlers::open_with::handle(ctx, sender_pid, &req, body, tx),
        OP_TAKE_OPEN_ARG if body.is_empty() => {
            handlers::take_open_arg::handle(ctx, sender_pid, &req, tx)
        }
        _ if body.is_empty() => {
            let _ = respond::status(sender_pid, &req, E_BAD_OP, tx);
        }
        _ => {
            let _ = respond::status(sender_pid, &req, E_INVAL, tx);
        }
    }
}
