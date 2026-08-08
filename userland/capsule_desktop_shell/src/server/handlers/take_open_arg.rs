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

use crate::protocol::Request;
use crate::server::handlers::launcher_request::lookup_pid;
use crate::server::respond;
use crate::state::apps::LAUNCHER_APPS;
use crate::state::Context;

pub fn handle(ctx: &mut Context, sender_pid: u32, req: &Request, tx: &mut [u8]) {
    let svc =
        LAUNCHER_APPS.iter().find(|a| lookup_pid(a.service) == Some(sender_pid)).map(|a| a.service);
    let path = match svc
        .and_then(|s| core::str::from_utf8(s).ok())
        .and_then(|s| ctx.pending_open.remove(s))
    {
        Some(p) => p,
        None => {
            let _ = respond::status(sender_pid, req, 0, tx);
            return;
        }
    };
    let _ = respond::payload(sender_pid, req, path.as_bytes(), tx);
}
