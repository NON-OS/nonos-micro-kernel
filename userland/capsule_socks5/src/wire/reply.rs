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

//! The SOCKS5 reply.

use super::{ATYP_IPV4, VER};

/// A reply is always ten bytes: header, IPv4 address type, a zero bound address
/// and a zero bound port.
pub const REPLY_LEN: usize = 10;

/// Encode a reply `[VER][REP][RSV][ATYP=IPv4][BND.ADDR=0.0.0.0][BND.PORT=0]` into
/// `out`, returning the length. The bound address is reported as zero: the stream
/// is tunneled through the mixnet, so there is no local socket address that would
/// mean anything to the client, and RFC 1928 permits a zero bind address.
pub fn reply(rep: u8, out: &mut [u8; REPLY_LEN]) -> usize {
    out[0] = VER;
    out[1] = rep;
    out[2] = 0x00; // RSV
    out[3] = ATYP_IPV4;
    out[4..REPLY_LEN].fill(0); // BND.ADDR (4) + BND.PORT (2)
    REPLY_LEN
}
