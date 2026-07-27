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

// NONOS std net: TCP/UDP over the net.sockets capsule and DNS over the
// net.dns capsule, via IPC. Connect/read/write, bind/accept and send/recv
// are wired; read/write timeouts and nonblocking mode are enforced through
// the IPC call deadline. peek, IPv6 and multicast stay unsupported (the
// userland stack is IPv4).
//
// The shared plumbing lives in `transport`; the std-facing socket types are
// split by concern into `tcp_stream`, `udp_socket`, `tcp_listener` and
// `dns`, and re-exported here.

mod dns;
mod socket;
mod tcp_listener;
mod tcp_stream;
mod transport;
mod udp_socket;

pub use dns::{LookupHost, lookup_host};
pub use socket::Socket;
pub use tcp_listener::TcpListener;
pub use tcp_stream::TcpStream;
pub use udp_socket::UdpSocket;
