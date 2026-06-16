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

use crate::protocol::{E_BAD_LEN, E_CLOSED, E_NO_SOCKET, E_OK, E_TIMEOUT, OP_SEND, SEGMENT_PAYLOAD_MAX};
use crate::server::handlers::io::u32_at;
use crate::server::parse_req::Request;
use crate::server::respond::respond;
use crate::state::TABLE;
use crate::tcp::State;

pub fn handle(sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    if body.len() < 4 || body.len() - 4 > SEGMENT_PAYLOAD_MAX {
        return status(sender_pid, req, E_BAD_LEN, tx);
    }
    let handle = match u32_at(body, 0) {
        Ok(h) => h,
        Err(_) => return status(sender_pid, req, E_BAD_LEN, tx),
    };
    let payload = &body[4..];
    let errno = {
        let mut table = TABLE.lock();
        match table.owned_mut(sender_pid, handle) {
            Some(e) if e.tcb.state == State::Established => {
                if e.enqueue_send(payload) {
                    crate::server::sender::drain_send(e);
                    E_OK
                } else {
                    E_TIMEOUT
                }
            }
            Some(_) => E_CLOSED,
            None => E_NO_SOCKET,
        }
    };
    let _ = respond(sender_pid, OP_SEND, errno, req.request_id, 0, tx);
}

fn status(sender_pid: u32, req: &Request, errno: u16, tx: &mut [u8]) {
    let _ = respond(sender_pid, OP_SEND, errno, req.request_id, 0, tx);
}
