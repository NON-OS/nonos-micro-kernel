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

use crate::clients::tcp;
use crate::protocol::{E_NO_HANDLE, E_NO_TRANSPORT, E_OK, E_TABLE_FULL, OP_ACCEPT};
use crate::server::handlers::io::u32_at;
use crate::server::parse_req::Request;
use crate::server::respond::respond;
use crate::sockets::{Kind, SocketKey, SOCKETS};
use crate::state;

pub fn handle(pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    let handle = match u32_at(body, 0) {
        Ok(h) => h,
        Err(e) => return status(pid, req, e, tx),
    };
    let Some(parent) = SOCKETS.with(SocketKey { pid, handle }, |s| s.transport_handle) else {
        return status(pid, req, E_NO_HANDLE, tx);
    };
    let child = match tcp::accept(state::tcp(), parent) {
        Ok(h) => h,
        Err(_) => return status(pid, req, E_NO_TRANSPORT, tx),
    };
    let Some(key) = SOCKETS.open(pid, Kind::Stream) else {
        let _ = tcp::close(state::tcp(), child);
        return status(pid, req, E_TABLE_FULL, tx);
    };
    SOCKETS.with(key, |s| {
        s.transport_handle = child;
        s.bound = true;
    });
    tx[20..24].copy_from_slice(&key.handle.to_le_bytes());
    respond(pid, OP_ACCEPT, E_OK, req.request_id, 4, tx);
}

fn status(pid: u32, req: &Request, errno: u16, tx: &mut [u8]) {
    respond(pid, OP_ACCEPT, errno, req.request_id, 0, tx);
}
