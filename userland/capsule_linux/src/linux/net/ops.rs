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
//!
//! Transcribed from userland/capsule_net_sockets/src/protocol/ops.rs. The
//! server is a separate binary and its protocol module is not a library,
//! so these cannot be imported; a number changed there and not here is a
//! wrong operation, which is why the source is named.

pub const OP_SOCKET: u16 = 2;
pub const OP_CONNECT: u16 = 6;
pub const OP_SEND: u16 = 7;
pub const OP_RECV: u16 = 8;
pub const OP_CLOSE: u16 = 9;
pub const OP_POLL: u16 = 13;

/// What OP_POLL reports: a recv would return data, a send would take it.
pub const POLL_READABLE: u8 = 1;
pub const POLL_WRITABLE: u8 = 2;
