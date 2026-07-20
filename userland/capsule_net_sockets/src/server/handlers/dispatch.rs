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

use super::{
    accept, bind, close, connect, getsockopt, health, listen, poll, recv, send, setsockopt, socket,
};
use crate::protocol::*;
use crate::server::parse_req::Request;

pub fn dispatch(pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) -> bool {
    match req.op {
        OP_HEALTHCHECK => health::handle(pid, req, tx),
        OP_SOCKET => socket::handle(pid, req, body, tx),
        OP_BIND => bind::handle(pid, req, body, tx),
        OP_LISTEN => listen::handle(pid, req, body, tx),
        OP_ACCEPT => accept::handle(pid, req, body, tx),
        OP_CONNECT => connect::handle(pid, req, body, tx),
        OP_CONNECT_NB => connect::handle_nb(pid, req, body, tx),
        OP_CONNECT_HOST => connect::handle_host(pid, req, body, tx),
        OP_SEND => send::handle(pid, req, body, tx),
        OP_RECV => recv::handle(pid, req, body, tx),
        OP_CLOSE => close::handle(pid, req, body, tx),
        OP_GETSOCKOPT => getsockopt::handle(pid, req, body, tx),
        OP_SETSOCKOPT => setsockopt::handle(pid, req, body, tx),
        OP_POLL => poll::handle(pid, req, body, tx),
        _ => return false,
    }
    true
}
