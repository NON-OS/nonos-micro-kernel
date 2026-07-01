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

use super::recv_drain::drain_stream;
use crate::protocol::{E_BAD_LEN, E_NO_SESSION, E_OK, E_RX_EMPTY, MIX_PAYLOAD_MAX, OP_RECV};
use crate::server::handlers::io::u32_at;
use crate::server::parse_req::Request;
use crate::server::respond::respond;
use crate::state::TABLE;

pub fn handle(pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    let session_id = match u32_at(body, 0) {
        Ok(id) => id,
        Err(e) => return respond(pid, OP_RECV, e, req.request_id, 0, tx),
    };
    if deliver_queued(pid, req, session_id, tx) {
        return;
    }
    drain_stream();
    if deliver_queued(pid, req, session_id, tx) {
        return;
    }
    respond(pid, OP_RECV, E_RX_EMPTY, req.request_id, 0, tx);
}

fn deliver_queued(pid: u32, req: &Request, id: u32, tx: &mut [u8]) -> bool {
    if tx.len() < 20 + MIX_PAYLOAD_MAX {
        respond(pid, OP_RECV, E_BAD_LEN, req.request_id, 0, tx);
        return true;
    }
    let msg = TABLE.lock().with_mut(pid, id, |s| s.pop());
    match msg {
        Some(Some(body)) if 20 + body.len() <= tx.len() => {
            tx[20..20 + body.len()].copy_from_slice(&body);
            respond(pid, OP_RECV, E_OK, req.request_id, body.len() as u32, tx);
            true
        }
        Some(_) => false,
        None => {
            respond(pid, OP_RECV, E_NO_SESSION, req.request_id, 0, tx);
            true
        }
    }
}
