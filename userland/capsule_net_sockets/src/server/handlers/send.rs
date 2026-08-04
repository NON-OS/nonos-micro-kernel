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

use crate::clients::{tcp, udp};
use crate::protocol::{E_NOT_CONNECTED, E_NO_HANDLE, E_NO_TRANSPORT, E_OK, OP_SEND};
use crate::server::handlers::io::u32_at;
use crate::server::handlers::mixnet_send::send_mixnet;
use crate::server::parse_req::Request;
use crate::server::respond::respond;
use crate::sockets::{Kind, Socket, SocketKey, SOCKETS};
use crate::state;

pub fn handle(pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    if body.len() < 4 {
        return status(pid, req, crate::protocol::E_BAD_LEN, tx);
    }
    let handle = match u32_at(body, 0) {
        Ok(handle) => handle,
        Err(e) => return status(pid, req, e, tx),
    };
    let key = SocketKey { pid, handle };
    let Some(sock) = SOCKETS.with(key, |s| *s) else {
        return status(pid, req, E_NO_HANDLE, tx);
    };
    let errno = send_socket(sock, &body[4..]);
    status(pid, req, errno, tx);
}

fn send_socket(sock: Socket, payload: &[u8]) -> u16 {
    match sock.kind {
        Kind::Stream if sock.transport_handle != 0 => {
            tcp::send(state::tcp(), sock.transport_handle, payload)
                .map(|_| E_OK)
                .map_or(E_NO_TRANSPORT, |errno| errno)
        }
        Kind::Datagram => match (sock.local, sock.remote) {
            (Some(l), Some(r)) => udp::send(state::udp(), l.port, r.ip, r.port, payload)
                .map(|_| E_OK)
                .map_or(E_NO_TRANSPORT, |errno| errno),
            _ => E_NOT_CONNECTED,
        },
        Kind::Mixnet if sock.transport_handle != 0 => match sock.remote {
            Some(r) => send_mixnet(sock.transport_handle, r.ip, r.port, payload),
            None => E_NOT_CONNECTED,
        },
        _ => E_NOT_CONNECTED,
    }
}

fn status(pid: u32, req: &Request, errno: u16, tx: &mut [u8]) {
    respond(pid, OP_SEND, errno, req.request_id, 0, tx);
}
