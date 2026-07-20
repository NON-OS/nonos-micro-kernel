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
use crate::protocol::{E_NOT_CONNECTED, E_NO_HANDLE, E_NO_TRANSPORT, E_OK, OP_POLL};
use crate::server::handlers::io::u32_at;
use crate::server::parse_req::Request;
use crate::server::respond::respond;
use crate::sockets::{Kind, Socket, SocketKey, SOCKETS};
use crate::state;

// Report one stream socket's readiness without draining it: the transport's
// non-consuming poll gives bit0 (a recv would return data) and bit1 (a send
// would accept data). The reply is a single byte, so a reactor can poll a
// whole wait-set cheaply.
pub fn handle(pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    let handle = match u32_at(body, 0) {
        Ok(handle) => handle,
        Err(e) => return status(pid, req, e, tx),
    };
    let key = SocketKey { pid, handle };
    let Some(sock) = SOCKETS.with(key, |s| *s) else {
        return status(pid, req, E_NO_HANDLE, tx);
    };
    match poll_socket(sock) {
        Ok(bits) => {
            if tx.len() > 20 {
                tx[20] = bits;
                respond(pid, OP_POLL, E_OK, req.request_id, 1, tx);
            } else {
                status(pid, req, E_NO_TRANSPORT, tx);
            }
        }
        Err(e) => status(pid, req, e, tx),
    }
}

fn poll_socket(sock: Socket) -> Result<u8, u16> {
    match sock.kind {
        Kind::Stream if sock.transport_handle != 0 => {
            tcp::poll(state::tcp(), sock.transport_handle).map_err(|_| E_NO_TRANSPORT)
        }
        _ => Err(E_NOT_CONNECTED),
    }
}

fn status(pid: u32, req: &Request, errno: u16, tx: &mut [u8]) {
    respond(pid, OP_POLL, errno, req.request_id, 0, tx);
}
