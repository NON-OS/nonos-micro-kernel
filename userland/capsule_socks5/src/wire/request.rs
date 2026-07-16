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

//! The SOCKS5 CONNECT request.

use super::{
    ATYP_DOMAIN, ATYP_IPV4, ATYP_IPV6, CMD_CONNECT, REP_ADDR_UNSUPP, REP_CMD_UNSUPP,
    REP_GENERAL_FAIL, VER,
};

/// The destination host of a CONNECT, borrowing the request bytes. A domain name
/// is passed through unresolved so the exit, not the client machine, does the DNS
/// lookup: resolving locally would leak the destination to the local network.
pub enum Host<'a> {
    V4([u8; 4]),
    V6([u8; 16]),
    Domain(&'a [u8]),
}

/// A parsed CONNECT request.
pub struct Connect<'a> {
    pub host: Host<'a>,
    pub port: u16,
}

/// The outcome of parsing a request buffer: it is a valid CONNECT, it needs more
/// bytes, or it must be rejected with the given reply code.
pub enum Parsed<'a> {
    /// Not enough bytes yet; read more and try again.
    Incomplete,
    /// Reject with this `REP_*` code, then close.
    Rejected(u8),
    /// A well-formed CONNECT to this destination.
    Connect(Connect<'a>),
}

/// Parse a SOCKS5 request `[VER][CMD][RSV][ATYP][ADDR][PORT]`. Only CONNECT with
/// a known address type is accepted; anything else is rejected with the matching
/// reply code. A truncated buffer returns `Incomplete` rather than failing.
pub fn parse_connect(req: &[u8]) -> Parsed<'_> {
    if req.len() < 4 {
        return Parsed::Incomplete;
    }
    if req[0] != VER {
        return Parsed::Rejected(REP_GENERAL_FAIL);
    }
    if req[1] != CMD_CONNECT {
        return Parsed::Rejected(REP_CMD_UNSUPP);
    }
    // req[2] is RSV, defined as 0x00 and ignored.
    let (host, addr_end) = match req[3] {
        ATYP_IPV4 => {
            if req.len() < 4 + 4 + 2 {
                return Parsed::Incomplete;
            }
            let mut a = [0u8; 4];
            a.copy_from_slice(&req[4..8]);
            (Host::V4(a), 8)
        }
        ATYP_IPV6 => {
            if req.len() < 4 + 16 + 2 {
                return Parsed::Incomplete;
            }
            let mut a = [0u8; 16];
            a.copy_from_slice(&req[4..20]);
            (Host::V6(a), 20)
        }
        ATYP_DOMAIN => {
            if req.len() < 5 {
                return Parsed::Incomplete;
            }
            let len = req[4] as usize;
            let end = 5 + len;
            if req.len() < end + 2 {
                return Parsed::Incomplete;
            }
            (Host::Domain(&req[5..end]), end)
        }
        _ => return Parsed::Rejected(REP_ADDR_UNSUPP),
    };
    let port = u16::from_be_bytes([req[addr_end], req[addr_end + 1]]);
    Parsed::Connect(Connect { host, port })
}
