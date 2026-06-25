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

use crate::protocol::udp::{E_BAD_LEN, E_NO_SOCKET, E_OK, MAGIC_NUDP, OP_UNBIND};
use crate::server::parse_req::Request;
use crate::server::respond::reply;
use crate::state;
use crate::udp_ports;

pub fn handle(sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    if body.len() < 2 {
        let _ = reply(sender_pid, MAGIC_NUDP, OP_UNBIND, E_BAD_LEN, req.request_id, &[], tx);
        return;
    }
    let local_port = u16::from_le_bytes([body[0], body[1]]);

    let sock_handle = match udp_ports::get(sender_pid, local_port) {
        Some(h) => h,
        None => {
            let _ = reply(sender_pid, MAGIC_NUDP, OP_UNBIND, E_NO_SOCKET, req.request_id, &[], tx);
            return;
        }
    };

    state::with_iface(|_iface, sockets, _dev| {
        sockets.get_mut::<udp::Socket>(sock_handle).close();
    });
    udp_ports::remove(sender_pid, local_port);

    let _ = reply(sender_pid, MAGIC_NUDP, OP_UNBIND, E_OK, req.request_id, &[], tx);
}
