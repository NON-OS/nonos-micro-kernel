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

//! The `net.ip` service: send and collect one IP payload at a time for a
//! named protocol. Only ICMP is carried today, which is what `ping` needs; the
//! protocol byte is on the wire so a second protocol does not need a new
//! service.

pub const MAGIC_NIP4: u32 = 0x4E49_5034;

/// Send one payload to a destination address. Body is dst[4], proto, payload.
pub const OP_SEND_PACKET: u16 = 4;
/// Collect one delivered payload for a protocol. Body is the protocol byte.
pub const OP_POLL_PACKET: u16 = 5;

pub const PROTO_ICMP: u8 = 1;

pub const E_OK: u16 = 0;
pub const E_BAD_LEN: u16 = 4;
/// The interface has no address yet, so nothing can be addressed from it.
pub const E_NO_CONFIG: u16 = 5;
/// No route to the destination.
pub const E_NO_ROUTE: u16 = 6;
/// The next hop has not resolved yet. The caller retries rather than failing,
/// since this is the normal state while ARP is in flight.
pub const E_NO_NEIGHBOUR: u16 = 7;
/// Nothing has arrived for this protocol.
pub const E_EMPTY: u16 = 11;
/// A protocol this service does not carry.
pub const E_BAD_PROTO: u16 = 13;

/// Echo requests are small; this bounds what one call can move in either
/// direction so a caller cannot make the capsule buffer without limit.
pub const MAX_PAYLOAD: usize = 512;
