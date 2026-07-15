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

use alloc::string::String;

use crate::protocol::{Request, E_INVAL, E_NOENT};
use crate::server::handlers::launcher_request;
use crate::server::respond;
use crate::state::apps::LAUNCHER_APPS;
use crate::state::Context;

pub fn handle(ctx: &mut Context, sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    if body.len() < 2 {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    }
    let nlen = u16::from_le_bytes([body[0], body[1]]) as usize;
    if body.len() < 2 + nlen {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    }
    let svc = &body[2..2 + nlen];
    let path = &body[2 + nlen..];
    let app = match LAUNCHER_APPS.iter().find(|a| a.service == svc) {
        Some(a) => a,
        None => {
            let _ = respond::status(sender_pid, req, E_NOENT, tx);
            return;
        }
    };
    if let (Ok(k), Ok(v)) = (core::str::from_utf8(svc), core::str::from_utf8(path)) {
        ctx.pending_open.insert(String::from(k), String::from(v));
    }
    let _ = launcher_request::request(app);
    let _ = respond::status(sender_pid, req, 0, tx);
}
