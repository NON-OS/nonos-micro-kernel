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

pub const OP_HEALTHCHECK: u16 = 1;
pub const OP_SOCKET: u16 = 2;
pub const OP_BIND: u16 = 3;
pub const OP_LISTEN: u16 = 4;
pub const OP_ACCEPT: u16 = 5;
pub const OP_CONNECT: u16 = 6;
pub const OP_SEND: u16 = 7;
pub const OP_RECV: u16 = 8;
pub const OP_CLOSE: u16 = 9;
pub const OP_GETSOCKOPT: u16 = 10;
pub const OP_SETSOCKOPT: u16 = 11;
pub const OP_POLL: u16 = 12;
pub const OP_SETFLAGS: u16 = 13;
pub const OP_GETFLAGS: u16 = 14;
pub const OP_SETTIMEOUT: u16 = 15;
pub const FLAG_NONBLOCK: u32 = 1;
pub const POLLIN: u16 = 1;
pub const POLLOUT: u16 = 2;
