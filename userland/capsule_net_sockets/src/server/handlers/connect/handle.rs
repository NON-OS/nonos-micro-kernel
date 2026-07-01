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

use crate::protocol::{E_BAD_ADDR, E_NO_HANDLE};
use crate::server::parse_req::Request;
use crate::sockets::{Kind, SocketKey, SOCKETS};

use super::{parse_body, status, update_datagram, update_mixnet, update_stream};

pub fn handle(pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    let (handle, ip, port) = match parse_body::parse_body(body) {
        Ok(v) => v,
        Err(e) => return status::status(pid, req, e, tx),
    };
    if port == 0 || ip == [0, 0, 0, 0] {
        return status::status(pid, req, E_BAD_ADDR, tx);
    }
    let key = SocketKey { pid, handle };
    let Some(sock) = SOCKETS.with(key, |s| *s) else {
        return status::status(pid, req, E_NO_HANDLE, tx);
    };
    match sock.kind {
        Kind::Datagram => update_datagram::update_datagram(pid, req, key, ip, port, tx),
        Kind::Mixnet => update_mixnet::update_mixnet(pid, req, key, ip, port, tx),
        Kind::Stream => update_stream::update_stream(pid, req, key, ip, port, tx),
    }
}
