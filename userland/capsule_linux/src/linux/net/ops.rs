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

//! The net.sockets opcodes this capsule uses.

pub const OP_SOCKET: u16 = 2;
pub const OP_CONNECT_HOST: u16 = 12;
pub const OP_CONNECT: u16 = 6;
pub const OP_SEND: u16 = 7;
pub const OP_RECV: u16 = 8;
pub const OP_CLOSE: u16 = 9;
pub const OP_POLL: u16 = 13;

/// The socket kinds the server offers: 1 stream, 2 datagram, 3 mixnet.
pub const KIND_MIXNET: u16 = 3;

/// The address family the server takes. It is not AF_INET: the number
/// is the server's own and the two only look alike.
pub const DOMAIN: u16 = 4;

/// What OP_POLL reports: a recv would return data, a send would take it.
pub const POLL_READABLE: u8 = 1;
pub const POLL_WRITABLE: u8 = 2;
