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

use crate::clients::udp;
use crate::protocol::{E_ALREADY_BOUND, E_BAD_ADDR, E_NO_HANDLE, E_NO_TRANSPORT, E_OK, OP_BIND};
use crate::server::handlers::io::{ip4_at, u16_at, u32_at};
use crate::server::parse_req::Request;
use crate::server::respond::respond;
use crate::sockets::{Kind, LocalAddr4, SocketKey, SOCKETS};
use crate::state;

pub fn handle(pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    let (handle, port) = match parse_body(body) {
        Ok(v) => v,
        Err(e) => return status(pid, req, e, tx),
    };
    let key = SocketKey { pid, handle };
    let Some(sock) = SOCKETS.with(key, |s| *s) else {
        return status(pid, req, E_NO_HANDLE, tx);
    };
    if port == 0 {
        return status(pid, req, E_BAD_ADDR, tx);
    }
    if sock.bound {
        return status(pid, req, E_ALREADY_BOUND, tx);
    }
    if sock.kind == Kind::Datagram && udp::bind(state::udp(), port).is_err() {
        return status(pid, req, E_NO_TRANSPORT, tx);
    }
    let updated = SOCKETS.with(key, |s| {
        s.local = Some(LocalAddr4 { port });
        s.bound = true;
    });
    if updated.is_none() && sock.kind == Kind::Datagram {
        let _ = udp::unbind(state::udp(), port);
    }
    status(pid, req, updated.map_or(E_NO_HANDLE, |_| E_OK), tx);
}

fn parse_body(body: &[u8]) -> Result<(u32, u16), u16> {
    let handle = u32_at(body, 0)?;
    let _ = ip4_at(body, 4)?;
    Ok((handle, u16_at(body, 8)?))
}

fn status(pid: u32, req: &Request, errno: u16, tx: &mut [u8]) {
    respond(pid, OP_BIND, errno, req.request_id, 0, tx);
}
