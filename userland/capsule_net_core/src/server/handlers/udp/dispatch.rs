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

use crate::protocol::udp::{E_BAD_OP, MAGIC_NUDP, OP_BIND, OP_RECV, OP_SEND, OP_UNBIND};
use crate::server::parse_req::Request;
use crate::server::respond::reply;

use super::{bind, recv, send, unbind};

pub fn dispatch(sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    match req.op {
        OP_BIND => bind::handle(sender_pid, req, body, tx),
        OP_UNBIND => unbind::handle(sender_pid, req, body, tx),
        OP_SEND => send::handle(sender_pid, req, body, tx),
        OP_RECV => recv::handle(sender_pid, req, body, tx),
        _ => {
            let _ = reply(sender_pid, MAGIC_NUDP, req.op, E_BAD_OP, req.request_id, &[], tx);
        }
    }
}
