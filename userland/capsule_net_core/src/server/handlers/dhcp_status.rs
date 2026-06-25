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

use crate::protocol::errno::{E_BAD_OP, E_OK};
use crate::protocol::ops::{MAGIC_NDHC, OP_LEASE_STATUS};
use crate::server::parse_req::Request;
use crate::server::respond::reply;
use crate::state;

pub fn dispatch(sender_pid: u32, req: &Request, tx: &mut [u8]) {
    match req.op {
        OP_LEASE_STATUS => lease_status(sender_pid, req, tx),
        _ => {
            let _ = reply(sender_pid, MAGIC_NDHC, req.op, E_BAD_OP, req.request_id, &[], tx);
        }
    }
}

pub fn encode_body(body: &mut [u8; 18]) {
    let lease = state::lease();
    match lease {
        Some(l) if l.bound => {
            body[0] = 3;
            body[1..5].copy_from_slice(&l.ip);
            body[5] = l.prefix;
            body[6..10].copy_from_slice(&l.gw);
            body[10..14].copy_from_slice(&l.dns);
            body[14..18].copy_from_slice(&l.secs.to_le_bytes());
        }
        _ => {
            body[0] = 1;
        }
    }
}

fn lease_status(sender_pid: u32, req: &Request, tx: &mut [u8]) {
    let mut body = [0u8; 18];
    encode_body(&mut body);
    let _ = reply(sender_pid, MAGIC_NDHC, OP_LEASE_STATUS, E_OK, req.request_id, &body, tx);
}
