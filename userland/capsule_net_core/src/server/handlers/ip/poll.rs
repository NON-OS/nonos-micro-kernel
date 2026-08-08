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

use smoltcp::wire::IpAddress;

use super::last_ident::last_ident;
use super::socket::with_icmp;
use crate::protocol::ip::*;
use crate::server::parse_req::Request;
use crate::server::respond::reply;

/// Collect one delivered ICMP message.
///
/// The reply carries the source address ahead of the payload so the caller can
/// tell which host answered without reparsing the IP header it never sees.
pub fn handle(sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    if body.is_empty() {
        return fail(sender_pid, req, E_BAD_LEN, tx);
    }
    if body[0] != PROTO_ICMP {
        return fail(sender_pid, req, E_BAD_PROTO, tx);
    }
    let Some(ident) = last_ident() else {
        return fail(sender_pid, req, E_EMPTY, tx);
    };

    let mut out = [0u8; 9 + MAX_PAYLOAD];
    let taken = with_icmp(ident, |sock| {
        if !sock.can_recv() {
            return 0usize;
        }
        match sock.recv() {
            Ok((payload, IpAddress::Ipv4(src))) => {
                let n = payload.len().min(MAX_PAYLOAD);
                out[0..4].copy_from_slice(src.as_bytes());
                out[8] = PROTO_ICMP;
                out[9..9 + n].copy_from_slice(&payload[..n]);
                9 + n
            }
            _ => 0,
        }
    });

    match taken {
        Some(n) if n > 0 => {
            let _ = reply(sender_pid, MAGIC_NIP4, req.op, E_OK, req.request_id, &out[..n], tx);
        }
        _ => fail(sender_pid, req, E_EMPTY, tx),
    }
}

fn fail(sender_pid: u32, req: &Request, code: u16, tx: &mut [u8]) {
    let _ = reply(sender_pid, MAGIC_NIP4, req.op, code, req.request_id, &[], tx);
}
