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

// Net service protocol constants: the net.sockets and net.dns service names
// and magics, the frame body offset and payload cap, the socket operation
// codes, and the kernel's timed-out errno.

pub(crate) const SK_NAME: &[u8] = b"net.sockets";
pub(crate) const SK_MAGIC: u32 = 0x4E53_4B54;
pub(crate) const DNS_NAME: &[u8] = b"net.dns";
pub(crate) const DNS_MAGIC: u32 = 0x4E44_4E53;
pub(crate) const BODY: usize = 20;
pub(crate) const MAX_PAYLOAD: usize = 1536;
pub(crate) const OP_SOCKET: u16 = 2;
pub(crate) const OP_BIND: u16 = 3;
pub(crate) const OP_LISTEN: u16 = 4;
pub(crate) const OP_ACCEPT: u16 = 5;
pub(crate) const OP_CONNECT: u16 = 6;
pub(crate) const OP_SEND: u16 = 7;
pub(crate) const OP_RECV: u16 = 8;
pub(crate) const OP_CLOSE: u16 = 9;

// The kernel's synchronous IPC call returns this when its deadline lapses.
pub(crate) const ERRNO_TIMEDOUT: i64 = -110;
