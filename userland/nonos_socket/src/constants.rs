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
//! Wire constants of the net.sockets protocol.

pub(crate) const SOCKETS_MAGIC: u32 = 0x4E53_4B54;
pub(crate) const OP_SOCKET: u16 = 2;
pub(crate) const OP_SEND: u16 = 7;
pub(crate) const OP_RECV: u16 = 8;
pub(crate) const OP_CLOSE: u16 = 9;
pub(crate) const OP_CONNECT_HOST: u16 = 12;
pub(crate) const SOCKET_FAMILY_IP4: u16 = 4;
pub(crate) const SOCKET_KIND_STREAM: u16 = 1;

/// Bytes of header before a request or reply body.
pub(crate) const HDR_LEN: usize = 20;
