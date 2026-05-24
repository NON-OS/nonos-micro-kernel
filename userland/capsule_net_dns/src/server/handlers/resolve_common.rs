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

use core::str;

use nonos_libc::mk_yield;

use crate::dns::{first_address, Answer, RCODE_NO_ERROR, RCODE_NXDOMAIN};
use crate::protocol::{E_NAME_INVALID, E_NXDOMAIN, E_SERVFAIL, E_TIMEOUT};
use crate::state::{local_port, next_xid, udp_port, upstream, DNS_PORT};
use crate::udp_client::{recv_from, send_to, UdpRecvError};

const RECV_TRIES: usize = 96;

pub fn name(body: &[u8]) -> Result<&str, u16> {
    if body.is_empty() || body.len() > 255 {
        return Err(E_NAME_INVALID);
    }
    str::from_utf8(body).map_err(|_| E_NAME_INVALID)
}

pub fn exchange(query: &[u8], xid: u16) -> Result<Answer, u16> {
    let upstream = upstream();
    send_to(udp_port(), local_port(), upstream, DNS_PORT, query).map_err(|_| E_TIMEOUT)?;
    for _ in 0..RECV_TRIES {
        match recv_from(udp_port(), local_port()) {
            Ok(d) if d.src == upstream && d.src_port == DNS_PORT => {
                return parse_response(&d.payload, xid);
            }
            Ok(_) | Err(UdpRecvError::Empty) => {
                mk_yield();
            }
            Err(_) => return Err(E_TIMEOUT),
        }
    }
    Err(E_TIMEOUT)
}

pub fn xid() -> u16 {
    next_xid()
}

fn parse_response(payload: &[u8], xid: u16) -> Result<Answer, u16> {
    let (hdr, answer) = first_address(payload).map_err(|_| E_SERVFAIL)?;
    if hdr.id != xid {
        return Err(E_TIMEOUT);
    }
    if hdr.rcode() == RCODE_NXDOMAIN {
        return Err(E_NXDOMAIN);
    }
    if hdr.rcode() != RCODE_NO_ERROR {
        return Err(E_SERVFAIL);
    }
    answer.ok_or(E_NXDOMAIN)
}
