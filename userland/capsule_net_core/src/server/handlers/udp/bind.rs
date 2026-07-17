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

use smoltcp::socket::udp;
use smoltcp::storage::PacketMetadata;

use crate::protocol::udp::{
    E_BAD_ADDR, E_BAD_LEN, E_BIND_FAILED, E_NO_SOCKET, E_OK, MAGIC_NUDP, OP_BIND,
};
use crate::server::parse_req::Request;
use crate::server::respond::reply;
use crate::state;
use crate::udp_ports;

enum BindOutcome {
    Ok,
    BindFailed,
    TableFull,
}

pub fn handle(sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    if body.len() < 2 {
        let _ = reply(sender_pid, MAGIC_NUDP, OP_BIND, E_BAD_LEN, req.request_id, &[], tx);
        return;
    }
    let local_port = u16::from_le_bytes([body[0], body[1]]);
    if local_port == 0 {
        let _ = reply(sender_pid, MAGIC_NUDP, OP_BIND, E_BAD_ADDR, req.request_id, &[], tx);
        return;
    }

    let outcome = state::with_iface(|_iface, sockets, _dev| {
        let rx =
            udp::PacketBuffer::new(alloc::vec![PacketMetadata::EMPTY; 16], alloc::vec![0u8; 4096]);
        let tx_buf =
            udp::PacketBuffer::new(alloc::vec![PacketMetadata::EMPTY; 16], alloc::vec![0u8; 4096]);
        let mut sock = udp::Socket::new(rx, tx_buf);
        if sock.bind(local_port).is_err() {
            return BindOutcome::BindFailed;
        }
        let handle = sockets.add(sock);
        if udp_ports::insert(sender_pid, local_port, handle) {
            BindOutcome::Ok
        } else {
            sockets.remove(handle);
            BindOutcome::TableFull
        }
    });

    let errno = match match outcome {
        Some(value) => value,
        None => BindOutcome::TableFull,
    } {
        BindOutcome::Ok => E_OK,
        BindOutcome::BindFailed => E_BIND_FAILED,
        BindOutcome::TableFull => E_NO_SOCKET,
    };
    let _ = reply(sender_pid, MAGIC_NUDP, OP_BIND, errno, req.request_id, &[], tx);
}
