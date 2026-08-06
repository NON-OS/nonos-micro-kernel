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

use super::{poll, send};
use crate::protocol::ip::{MAGIC_NIP4, OP_POLL_PACKET, OP_SEND_PACKET};
use crate::server::parse_req::Request;
use crate::server::respond::reply;

const E_BAD_OP: u16 = 3;

pub fn dispatch(sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    match req.op {
        OP_SEND_PACKET => send::handle(sender_pid, req, body, tx),
        OP_POLL_PACKET => poll::handle(sender_pid, req, body, tx),
        _ => {
            let _ = reply(sender_pid, MAGIC_NIP4, req.op, E_BAD_OP, req.request_id, &[], tx);
        }
    }
}
