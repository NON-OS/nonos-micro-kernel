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

use crate::protocol::tcp::{MAGIC_NTCP, OP_CONNECT};
use crate::server::handlers::tcp::connect::{endpoint, open_socket, reply_outcome};
use crate::server::parse_req::Request;
use crate::server::respond::reply;

pub fn handle(sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    let endpoint = match endpoint::parse(body) {
        Ok(endpoint) => endpoint,
        Err(errno) => {
            let _ = reply(sender_pid, MAGIC_NTCP, OP_CONNECT, errno, req.request_id, &[], tx);
            return;
        }
    };
    let outcome = open_socket::open_socket(sender_pid, endpoint);
    reply_outcome::reply_outcome(sender_pid, req.request_id, outcome, tx);
}
