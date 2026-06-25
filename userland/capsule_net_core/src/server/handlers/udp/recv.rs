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
use smoltcp::wire::IpAddress;

use crate::protocol::udp::{E_BAD_LEN, E_NO_SOCKET, E_OK, E_RX_EMPTY, MAGIC_NUDP, OP_RECV};
use crate::server::parse_req::{Request, IPC_BUF_MAX};
use crate::server::respond::reply;
use crate::state;
use crate::udp_ports;

pub fn handle(sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    if body.len() < 2 {
        let _ = reply(sender_pid, MAGIC_NUDP, OP_RECV, E_BAD_LEN, req.request_id, &[], tx);
        return;
    }
    let local_port = u16::from_le_bytes([body[0], body[1]]);

    let sock_handle = match udp_ports::get(sender_pid, local_port) {
        Some(h) => h,
        None => {
            let _ = reply(sender_pid, MAGIC_NUDP, OP_RECV, E_NO_SOCKET, req.request_id, &[], tx);
            return;
        }
    };

    let result = state::with_iface(|_iface, sockets, _dev| {
        let sock = sockets.get_mut::<udp::Socket>(sock_handle);
        sock.recv().ok().map(|(payload, meta)| {
            let IpAddress::Ipv4(v4) = meta.endpoint.addr;
            let src_ip = v4.0;
            let src_port = meta.endpoint.port;
            let len = payload.len().min(IPC_BUF_MAX - 6);
            let mut out = alloc::vec![0u8; 6 + len];
            out[0..4].copy_from_slice(&src_ip);
            out[4..6].copy_from_slice(&src_port.to_le_bytes());
            out[6..].copy_from_slice(&payload[..len]);
            out
        })
    });

    match result.flatten() {
        Some(datagram) => {
            let _ = reply(sender_pid, MAGIC_NUDP, OP_RECV, E_OK, req.request_id, &datagram, tx);
        }
        None => {
            let _ = reply(sender_pid, MAGIC_NUDP, OP_RECV, E_RX_EMPTY, req.request_id, &[], tx);
        }
    }
}
