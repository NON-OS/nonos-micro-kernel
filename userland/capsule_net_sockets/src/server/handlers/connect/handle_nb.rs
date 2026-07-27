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

use crate::protocol::{E_BAD_ADDR, E_NO_HANDLE, OP_CONNECT_NB};
use crate::server::parse_req::Request;
use crate::server::respond::respond;
use crate::sockets::{Kind, SocketKey, SOCKETS};

use super::{parse_body, update_datagram, update_mixnet, update_stream_nb};

// Non-blocking counterpart of `handle`. A stream socket initiates its handshake
// and returns at once (the caller confirms via OP_POLL); datagram and mixnet
// sockets have no handshake, so they set their remote exactly as in the
// blocking path.
pub fn handle(pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    let (handle, ip, port) = match parse_body::parse_body(body) {
        Ok(v) => v,
        Err(e) => {
            respond(pid, OP_CONNECT_NB, e, req.request_id, 0, tx);
            return;
        }
    };
    if port == 0 || ip == [0, 0, 0, 0] {
        respond(pid, OP_CONNECT_NB, E_BAD_ADDR, req.request_id, 0, tx);
        return;
    }
    let key = SocketKey { pid, handle };
    let Some(sock) = SOCKETS.with(key, |s| *s) else {
        respond(pid, OP_CONNECT_NB, E_NO_HANDLE, req.request_id, 0, tx);
        return;
    };
    match sock.kind {
        Kind::Datagram => update_datagram::update_datagram(pid, req, key, ip, port, tx),
        Kind::Mixnet => update_mixnet::update_mixnet(pid, req, key, ip, port, tx),
        Kind::Stream => update_stream_nb::update_stream_nb(pid, req, key, ip, port, tx),
    }
}
