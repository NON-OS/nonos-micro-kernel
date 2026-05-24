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

use crate::clients::nym;
use crate::protocol::{E_BAD_LEN, E_NO_HANDLE, E_NO_TRANSPORT, E_OK, OP_SETSOCKOPT};
use crate::server::handlers::io::{u16_at, u32_at};
use crate::server::parse_req::Request;
use crate::server::respond::respond;
use crate::sockets::{Kind, Socket, SocketKey, SOCKETS};
use crate::state;

const LEVEL_MIXNET: u16 = 3;
const OPT_COVER_TICK: u16 = 1;

pub fn handle(pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    let handle = match u32_at(body, 0) {
        Ok(handle) => handle,
        Err(e) => {
            let _ = respond(pid, OP_SETSOCKOPT, e, req.request_id, 0, tx);
            return;
        }
    };
    let level = match u16_at(body, 4) {
        Ok(level) => level,
        Err(e) => {
            let _ = respond(pid, OP_SETSOCKOPT, e, req.request_id, 0, tx);
            return;
        }
    };
    let opt = match u16_at(body, 6) {
        Ok(opt) => opt,
        Err(e) => {
            let _ = respond(pid, OP_SETSOCKOPT, e, req.request_id, 0, tx);
            return;
        }
    };
    let key = SocketKey { pid, handle };
    let errno = SOCKETS.with(key, |s| apply(s, level, opt)).map_or(E_NO_HANDLE, |e| e);
    let _ = respond(pid, OP_SETSOCKOPT, errno, req.request_id, 0, tx);
}

fn apply(sock: &mut Socket, level: u16, opt: u16) -> u16 {
    if level != LEVEL_MIXNET || opt != OPT_COVER_TICK {
        return E_BAD_LEN;
    }
    if sock.kind != Kind::Mixnet || sock.transport_handle == 0 {
        return E_NO_TRANSPORT;
    }
    nym::cover(state::nym(), sock.transport_handle).map_or(E_NO_TRANSPORT, |_| E_OK)
}
