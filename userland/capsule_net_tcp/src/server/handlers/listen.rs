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

use crate::protocol::{E_BAD_ADDR, E_BAD_LEN, E_OK, E_PORT_IN_USE, OP_LISTEN};
use crate::server::handlers::io::u16_at;
use crate::server::parse_req::Request;
use crate::server::respond::respond;
use crate::state::{local_ip, TABLE};
use crate::tcp::{Endpoint4, Tcb};

pub fn handle(sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    let port = match u16_at(body, 0) {
        Ok(p) => p,
        Err(_) => return status(sender_pid, req, E_BAD_LEN, tx),
    };
    if port == 0 {
        return status(sender_pid, req, E_BAD_ADDR, tx);
    }
    let local = Endpoint4 { ip: local_ip(), port };
    let mut table = TABLE.lock();
    if table.listener_for_mut(port).is_some() {
        return status(sender_pid, req, E_PORT_IN_USE, tx);
    }
    match table.insert(sender_pid, 0, Tcb::listen(local)) {
        Ok(handle) => answer(sender_pid, req, handle, tx),
        Err(_) => status(sender_pid, req, E_PORT_IN_USE, tx),
    }
}

fn answer(sender_pid: u32, req: &Request, handle: u32, tx: &mut [u8]) {
    tx[20..24].copy_from_slice(&handle.to_le_bytes());
    let _ = respond(sender_pid, OP_LISTEN, E_OK, req.request_id, 4, tx);
}

fn status(sender_pid: u32, req: &Request, errno: u16, tx: &mut [u8]) {
    let _ = respond(sender_pid, OP_LISTEN, errno, req.request_id, 0, tx);
}
