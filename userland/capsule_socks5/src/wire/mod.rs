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

//! The SOCKS5 wire codec (RFC 1928): the method-selection handshake and the
//! CONNECT request and reply. Parsing is over untrusted client bytes, so it is
//! bounds-checked, allocation-free and never panics; a short buffer reports
//! "incomplete" so the server reads more rather than guessing. The transport that
//! carries the tunneled stream (the Nym mixnet) is separate; this module only
//! speaks the protocol a SOCKS5 client on the loopback expects.

mod greeting;
mod reply;
mod request;

pub use greeting::{method_reply, offers_no_auth};
pub use reply::{reply, REPLY_LEN};
pub use request::{parse_connect, Connect, Host, Parsed};

/// Protocol version. Only 5 is spoken.
pub const VER: u8 = 0x05;

/// No authentication required.
pub const METHOD_NONE: u8 = 0x00;
/// No acceptable authentication method: the client must disconnect.
pub const METHOD_NO_ACCEPT: u8 = 0xFF;

/// Establish a TCP connection to the destination. The only command supported: a
/// privacy proxy has no use for BIND or UDP ASSOCIATE.
pub const CMD_CONNECT: u8 = 0x01;

/// Address is a 4-byte IPv4 address.
pub const ATYP_IPV4: u8 = 0x01;
/// Address is a length-prefixed domain name.
pub const ATYP_DOMAIN: u8 = 0x03;
/// Address is a 16-byte IPv6 address.
pub const ATYP_IPV6: u8 = 0x04;

/// Reply codes (the `REP` field).
pub const REP_OK: u8 = 0x00;
pub const REP_GENERAL_FAIL: u8 = 0x01;
/// No route to the network the destination is on. Used when the mixnet itself
/// is not reachable, which is a different fault from an exit that will not take
/// the destination, and a client that cannot tell them apart cannot be debugged
/// on a machine with no console.
pub const REP_NET_UNREACH: u8 = 0x03;
pub const REP_HOST_UNREACH: u8 = 0x04;
pub const REP_CONN_REFUSED: u8 = 0x05;
pub const REP_CMD_UNSUPP: u8 = 0x07;
pub const REP_ADDR_UNSUPP: u8 = 0x08;
