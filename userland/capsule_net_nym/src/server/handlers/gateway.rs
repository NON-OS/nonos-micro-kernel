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

use crate::gateway_client;
use crate::protocol::{E_GATEWAY_PROTO, E_NO_TCP, E_OK, E_PERM, OP_SET_GATEWAY};
use crate::server::authz::admin;
use crate::server::handlers::io::{ip4_at, u16_at};
use crate::server::parse_req::Request;
use crate::server::respond::respond;
use crate::setup;
use crate::state::{Gateway, Transport, TABLE};

pub fn handle(pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    if !admin(pid) {
        return respond(pid, OP_SET_GATEWAY, E_PERM, req.request_id, 0, tx);
    }
    let gateway = match parse_gateway(body) {
        Ok(g) => g,
        Err(e) => return respond(pid, OP_SET_GATEWAY, e, req.request_id, 0, tx),
    };
    let tcp_port = setup::tcp_port();
    if tcp_port == 0 {
        return respond(pid, OP_SET_GATEWAY, E_NO_TCP, req.request_id, 0, tx);
    }
    let gateway = match gateway_client::connect(tcp_port, gateway) {
        Ok(gateway) => gateway,
        Err(e) => {
            let errno = if e == E_GATEWAY_PROTO { E_GATEWAY_PROTO } else { E_NO_TCP };
            return respond(pid, OP_SET_GATEWAY, errno, req.request_id, 0, tx);
        }
    };
    if let Some(old) = TABLE.lock().set_gateway(gateway) {
        if gateway_client::close(tcp_port, old).is_err() {
            return respond(pid, OP_SET_GATEWAY, E_NO_TCP, req.request_id, 0, tx);
        }
    }
    respond(pid, OP_SET_GATEWAY, E_OK, req.request_id, 0, tx);
}

fn parse_gateway(body: &[u8]) -> Result<Gateway, u16> {
    let ip = ip4_at(body, 0)?;
    let port = u16_at(body, 4)?;
    let mode = match body.get(6).copied() {
        Some(mode) => mode,
        None => 1,
    };
    let transport = match mode {
        0 => Transport::RawTcp,
        1 => Transport::WebSocket,
        _ => return Err(E_GATEWAY_PROTO),
    };
    // Optional so existing callers keep working; supplying it is what turns
    // the registration handshake on.
    let mut identity = [0u8; 32];
    if body.len() >= 7 + 32 {
        identity.copy_from_slice(&body[7..7 + 32]);
    }
    Ok(Gateway { ip, port, stream: 0, transport, identity, shared_key: [0u8; 32] })
}
