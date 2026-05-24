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

use crate::protocol::{E_NO_HANDLE, E_OK, OP_GETSOCKOPT};
use crate::server::handlers::io::u32_at;
use crate::server::parse_req::Request;
use crate::server::respond::respond;
use crate::sockets::{Kind, Socket, SocketKey, SOCKETS};

pub fn handle(pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    let handle = match u32_at(body, 0) {
        Ok(handle) => handle,
        Err(e) => {
            let _ = respond(pid, OP_GETSOCKOPT, e, req.request_id, 0, tx);
            return;
        }
    };
    let key = SocketKey { pid, handle };
    let _ = match SOCKETS.with(key, |s| write_status(s, &mut tx[20..36])) {
        Some(()) => respond(pid, OP_GETSOCKOPT, E_OK, req.request_id, 16, tx),
        None => respond(pid, OP_GETSOCKOPT, E_NO_HANDLE, req.request_id, 0, tx),
    };
}

fn write_status(sock: &Socket, out: &mut [u8]) {
    out[0..4].copy_from_slice(&kind(sock.kind).to_le_bytes());
    out[4..8].copy_from_slice(&flags(sock).to_le_bytes());
    out[8..12].copy_from_slice(&sock.transport_handle.to_le_bytes());
    out[12..16].fill(0);
}

fn kind(kind: Kind) -> u32 {
    match kind {
        Kind::Stream => 1,
        Kind::Datagram => 2,
        Kind::Mixnet => 3,
    }
}

fn flags(sock: &Socket) -> u32 {
    (sock.bound as u32)
        | ((sock.listening as u32) << 1)
        | ((sock.remote.is_some() as u32) << 2)
        | ((sock.transport_handle != 0) as u32) << 3
}
