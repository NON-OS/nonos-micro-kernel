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

use crate::dns::build_a_query;
use crate::protocol::{E_NAME_INVALID, E_OK, E_SERVFAIL, OP_RESOLVE_A};
use crate::server::handlers::resolve_common::{exchange, name, xid};
use crate::server::parse_req::Request;
use crate::server::respond::respond;
use crate::state::{now_ms, CACHE};

pub fn handle(sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    let qname = match name(body) {
        Ok(v) => v,
        Err(e) => return status(sender_pid, req, e, tx),
    };
    let now = now_ms();
    if let Some(ip) = CACHE.lock().lookup(qname, now) {
        return answer(sender_pid, req, ip, tx);
    }
    let xid = match xid() {
        Some(v) => v,
        None => return status(sender_pid, req, E_SERVFAIL, tx),
    };
    let mut query = [0u8; 512];
    let len = match build_a_query(xid, qname, &mut query) {
        Ok(n) => n,
        Err(_) => return status(sender_pid, req, E_NAME_INVALID, tx),
    };
    match exchange(&query[..len], xid).ok().and_then(|a| a.ipv4.map(|ip| (a, ip))) {
        Some((a, ip)) => {
            CACHE.lock().insert(qname, ip, (a.ttl as u64).saturating_mul(1000), now);
            answer(sender_pid, req, ip, tx);
        }
        None => status(sender_pid, req, E_SERVFAIL, tx),
    }
}

fn answer(sender_pid: u32, req: &Request, ip: [u8; 4], tx: &mut [u8]) {
    tx[20..24].copy_from_slice(&ip);
    let _ = respond(sender_pid, OP_RESOLVE_A, E_OK, req.request_id, 4, tx);
}

fn status(sender_pid: u32, req: &Request, errno: u16, tx: &mut [u8]) {
    let _ = respond(sender_pid, OP_RESOLVE_A, errno, req.request_id, 0, tx);
}
