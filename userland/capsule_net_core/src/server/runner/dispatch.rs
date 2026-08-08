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

use crate::protocol::dns::MAGIC_NDNS;
use crate::protocol::errno::E_BAD_MAGIC;
use crate::protocol::ip::MAGIC_NIP4;
use crate::protocol::ops::MAGIC_NDHC;
use crate::protocol::tcp::MAGIC_NTCP;
use crate::protocol::udp::MAGIC_NUDP;
use crate::server::handlers::health::{handle as health_handle, OP_HEALTHCHECK};
use crate::server::parse_req::Request;
use crate::server::respond::reply;

pub fn dispatch(sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    match req.op {
        OP_HEALTHCHECK => health_handle(sender_pid, req, tx),
        _ => match req.magic {
            MAGIC_NDHC => crate::server::handlers::dhcp_status::dispatch(sender_pid, req, tx),
            MAGIC_NTCP => crate::server::handlers::tcp::dispatch(sender_pid, req, body, tx),
            MAGIC_NUDP => crate::server::handlers::udp::dispatch(sender_pid, req, body, tx),
            MAGIC_NDNS => crate::server::handlers::dns::dispatch(sender_pid, req, body, tx),
            MAGIC_NIP4 => crate::server::handlers::ip::dispatch(sender_pid, req, body, tx),
            _ => {
                let _ = reply(sender_pid, req.magic, req.op, E_BAD_MAGIC, req.request_id, &[], tx);
            }
        },
    }
}
