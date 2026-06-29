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

use nonos_libc::{mk_time_millis, mk_yield};

use crate::clients::{nym, tcp, udp};
use crate::protocol::{E_NOT_CONNECTED, E_NO_HANDLE, E_NO_TRANSPORT, E_OK, E_WOULD_BLOCK, OP_RECV};
use crate::server::handlers::io::u32_at;
use crate::server::parse_req::Request;
use crate::server::respond::respond;
use crate::sockets::{Kind, Socket, SocketKey, SOCKETS};
use crate::state;

pub fn handle(pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    let handle = match u32_at(body, 0) {
        Ok(handle) => handle,
        Err(e) => return status(pid, req, e, tx),
    };
    let key = SocketKey { pid, handle };
    let Some(sock) = SOCKETS.with(key, |s| *s) else {
        return status(pid, req, E_NO_HANDLE, tx);
    };
    let buffered = crate::sockets::stash::take(pid, handle, &mut tx[20..]);
    if buffered > 0 {
        let _ = respond(pid, OP_RECV, E_OK, req.request_id, buffered as u32, tx);
        return;
    }
    let deadline = mk_time_millis().saturating_add(sock.timeout_ms as i64);
    loop {
        match recv_socket(sock, &mut tx[20..]) {
            Ok(n) => {
                let _ = respond(pid, OP_RECV, E_OK, req.request_id, n as u32, tx);
                return;
            }
            Err(E_WOULD_BLOCK) if sock.timeout_ms > 0 && mk_time_millis() < deadline => {
                mk_yield();
            }
            Err(e) => return status(pid, req, e, tx),
        }
    }
}

fn map_recv_err(e: u16, empty: u16) -> u16 {
    if e == empty { E_WOULD_BLOCK } else { E_NO_TRANSPORT }
}

fn recv_socket(sock: Socket, out: &mut [u8]) -> Result<usize, u16> {
    match sock.kind {
        Kind::Stream if sock.transport_handle != 0 => {
            tcp::recv(state::tcp(), sock.transport_handle, out)
                .map_err(|e| map_recv_err(e, tcp::RX_EMPTY))
        }
        Kind::Datagram => {
            let Some(local) = sock.local else { return Err(E_NOT_CONNECTED) };
            udp::recv(state::udp(), local.port, out)
                .map_err(|e| map_recv_err(e, udp::RX_EMPTY))
        }
        Kind::Mixnet if sock.transport_handle != 0 => {
            nym::recv(state::nym(), sock.transport_handle, out).map_err(|_| E_NO_TRANSPORT)
        }
        _ => Err(E_NOT_CONNECTED),
    }
}

fn status(pid: u32, req: &Request, errno: u16, tx: &mut [u8]) {
    respond(pid, OP_RECV, errno, req.request_id, 0, tx);
}
