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
pub const OP_CONNECT_HOST: u16 = 12;
// Non-consuming readiness poll for a stream socket: replies one byte (bit0
// readable, bit1 writable). The PAL async reactor waits on many sockets by
// polling each without draining it.
pub const OP_POLL: u16 = 13;
// Non-blocking stream connect: initiate the handshake, install the transport,
// and reply at once without waiting for establishment. The caller confirms the
// connection via OP_POLL writability. OP_CONNECT (blocking) stays for the
// synchronous PAL path; a reactor-driven client (mio/tokio) uses this so a
// connect never stalls the whole runtime.
pub const OP_CONNECT_NB: u16 = 14;
