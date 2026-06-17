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

use crate::dns::build_aaaa_query;
use crate::protocol::{E_NAME_INVALID, E_OK, E_SERVFAIL, OP_RESOLVE_AAAA};
use crate::server::handlers::resolve_common::{exchange, name, xid};
use crate::server::parse_req::Request;
use crate::server::respond::respond;

pub fn handle(sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    let qname = match name(body) {
        Ok(v) => v,
        Err(e) => return status(sender_pid, req, e, tx),
    };
    let xid = match xid() {
        Some(v) => v,
        None => return status(sender_pid, req, E_SERVFAIL, tx),
    };
    let mut query = [0u8; 512];
    let len = match build_aaaa_query(xid, qname, &mut query) {
        Ok(n) => n,
        Err(_) => return status(sender_pid, req, E_NAME_INVALID, tx),
    };
    match exchange(&query[..len], xid).ok().and_then(|a| a.ipv6) {
        Some(ip) => answer(sender_pid, req, ip, tx),
        None => status(sender_pid, req, E_SERVFAIL, tx),
    }
}

fn answer(sender_pid: u32, req: &Request, ip: [u8; 16], tx: &mut [u8]) {
    tx[20..36].copy_from_slice(&ip);
    let _ = respond(sender_pid, OP_RESOLVE_AAAA, E_OK, req.request_id, 16, tx);
}

fn status(sender_pid: u32, req: &Request, errno: u16, tx: &mut [u8]) {
    let _ = respond(sender_pid, OP_RESOLVE_AAAA, errno, req.request_id, 0, tx);
}
