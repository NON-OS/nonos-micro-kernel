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

use crate::clients::wire::lookup_pid;
use crate::protocol::{read_u32, Request, E_INVAL, E_ACCES};
use crate::server::respond;
use crate::state::Context;

const E_BUSY: i32 = -16;
const REQ_LEN: usize = 8;

const GRABBERS: [&[u8]; 3] = [b"app.boot_splash", b"app.setup_wizard", b"app.input_probe"];

fn is_trusted_grabber(sender_pid: u32) -> bool {
    GRABBERS.iter().any(|name| lookup_pid(name) == Some(sender_pid))
}

pub fn handle(ctx: &mut Context, sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    if !is_trusted_grabber(sender_pid) {
        let _ = respond::status(sender_pid, req, E_ACCES, tx);
        return;
    }
    if body.len() != REQ_LEN {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    }
    let Some(kind_mask) = read_u32(body, 0) else {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    };
    if kind_mask == 0 {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    }
    if !ctx.grabs.request(sender_pid, kind_mask) {
        let _ = respond::status(sender_pid, req, E_BUSY, tx);
        return;
    }
    let _ = respond::status(sender_pid, req, 0, tx);
}
