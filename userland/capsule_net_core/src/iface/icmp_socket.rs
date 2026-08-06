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

use smoltcp::iface::{SocketHandle, SocketSet};
use smoltcp::socket::icmp;

/// Four echo exchanges in flight is what `ping` uses per run, so a handful of
/// slots covers the normal case without letting a stalled reader grow the
/// queue.
const SLOTS: usize = 8;
const PAYLOAD: usize = 512;

/// Create the ICMP socket bound to `ident`.
///
/// Binding by identifier is what makes the kernel deliver only the replies
/// that answer this socket's own echo requests, instead of every ICMP message
/// that reaches the interface.
pub fn install_icmp_socket(sockets: &mut SocketSet<'static>, ident: u16) -> SocketHandle {
    let rx = icmp::PacketBuffer::new(
        alloc::vec![icmp::PacketMetadata::EMPTY; SLOTS],
        alloc::vec![0u8; SLOTS * PAYLOAD],
    );
    let tx = icmp::PacketBuffer::new(
        alloc::vec![icmp::PacketMetadata::EMPTY; SLOTS],
        alloc::vec![0u8; SLOTS * PAYLOAD],
    );
    let mut socket = icmp::Socket::new(rx, tx);
    // A bind failure here would mean the identifier was zero, which the caller
    // has already rejected.
    let _ = socket.bind(icmp::Endpoint::Ident(ident));
    sockets.add(socket)
}
