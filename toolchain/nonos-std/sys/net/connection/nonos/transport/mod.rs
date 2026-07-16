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

// Shared plumbing for the net backend: protocol constants, the syscall ABI
// stubs, the request/reply call, socket primitives, and the address and
// duration helpers. The std-facing socket types live in the sibling
// tcp_stream/, udp_socket/, tcp_listener and dns modules.

mod addr;
mod consts;
mod duration;
mod err;
mod ipc;
mod socket;
mod syscall;

pub(crate) use addr::{endpoint, unspecified, v4_parts};
pub(crate) use consts::{
    BODY, DNS_MAGIC, DNS_NAME, MAX_PAYLOAD, OP_ACCEPT, OP_BIND, OP_CONNECT, OP_LISTEN, OP_RECV,
    OP_SEND,
};
pub(crate) use duration::{dur_to_ms, ms_to_dur};
pub(crate) use err::{err, read_u32};
pub(crate) use ipc::{ipc, sk, sk_timed};
pub(crate) use socket::{close, open_socket, recv_on, send_on};
