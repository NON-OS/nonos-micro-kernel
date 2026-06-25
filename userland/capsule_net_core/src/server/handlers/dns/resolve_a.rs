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

use nonos_libc::mk_time_millis;
use smoltcp::socket::dns::{GetQueryResultError, QueryHandle, Socket as DnsSocket};
use smoltcp::wire::{DnsQueryType, IpAddress};

use crate::iface::poll;
use crate::protocol::dns::{E_NAME_INVALID, E_NO_LEASE, E_OK, E_SERVFAIL, MAGIC_NDNS, OP_RESOLVE_A};
use crate::server::parse_req::Request;
use crate::server::respond::reply;
use crate::state;

const TIMEOUT_MS: u64 = 3000;

pub fn handle(sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    let name = match core::str::from_utf8(body) {
        Ok(s) if !s.is_empty() => s,
        _ => return err(sender_pid, req, E_NAME_INVALID, tx),
    };

    let qh: QueryHandle = match state::with_dns(|iface, sockets, dns_handle| {
        let cx = iface.context();
        sockets.get_mut::<DnsSocket>(dns_handle).start_query(cx, name, DnsQueryType::A)
    }) {
        Some(Ok(h)) => h,
        Some(Err(_)) => return err(sender_pid, req, E_NAME_INVALID, tx),
        None => return err(sender_pid, req, E_NO_LEASE, tx),
    };

    let deadline = mk_time_millis() as u64 + TIMEOUT_MS;

    loop {
        poll::pump();
        if mk_time_millis() as u64 >= deadline {
            err(sender_pid, req, E_SERVFAIL, tx);
            return;
        }

        let result = state::with_dns(|_iface, sockets, dns_handle| {
            sockets.get_mut::<DnsSocket>(dns_handle).get_query_result(qh)
        });

        match result {
            Some(Ok(addrs)) => {
                let ip = addrs.iter().find_map(|a| match a {
                    IpAddress::Ipv4(v4) => Some(v4.0),
                });
                match ip {
                    Some(octets) => {
                        let _ = reply(sender_pid, MAGIC_NDNS, OP_RESOLVE_A, E_OK, req.request_id, &octets, tx);
                    }
                    None => err(sender_pid, req, E_SERVFAIL, tx),
                }
                return;
            }
            Some(Err(GetQueryResultError::Pending)) => continue,
            _ => {
                err(sender_pid, req, E_SERVFAIL, tx);
                return;
            }
        }
    }
}

fn err(sender_pid: u32, req: &Request, errno: u16, tx: &mut [u8]) {
    let _ = reply(sender_pid, MAGIC_NDNS, OP_RESOLVE_A, errno, req.request_id, &[], tx);
}
