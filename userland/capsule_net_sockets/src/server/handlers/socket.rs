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

use crate::protocol::{E_BAD_FAMILY, E_BAD_KIND, E_OK, E_TABLE_FULL, OP_SOCKET};
use crate::server::handlers::io::u16_at;
use crate::server::parse_req::Request;
use crate::server::respond::respond;
use crate::sockets::{Kind, SOCKETS};

pub fn handle(pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    match u16_at(body, 0) {
        Ok(4) => {}
        Ok(_) => return status(pid, req, E_BAD_FAMILY, tx),
        Err(e) => return status(pid, req, e, tx),
    }
    let kind = match u16_at(body, 2) {
        Ok(1) => Kind::Stream,
        Ok(2) => Kind::Datagram,
        Ok(3) => Kind::Mixnet,
        Ok(_) => return status(pid, req, E_BAD_KIND, tx),
        Err(e) => return status(pid, req, e, tx),
    };
    match SOCKETS.open(pid, kind) {
        Some(key) => {
            tx[20..24].copy_from_slice(&key.handle.to_le_bytes());
            respond(pid, OP_SOCKET, E_OK, req.request_id, 4, tx);
        }
        None => status(pid, req, E_TABLE_FULL, tx),
    }
}

fn status(pid: u32, req: &Request, errno: u16, tx: &mut [u8]) {
    respond(pid, OP_SOCKET, errno, req.request_id, 0, tx);
}
