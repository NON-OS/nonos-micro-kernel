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

use smoltcp::socket::tcp;

use crate::handles;
use crate::protocol::tcp::{E_BAD_LEN, E_NO_SOCKET, E_OK, E_RX_EMPTY, MAGIC_NTCP, OP_RECV};
use crate::server::handlers::tcp::recv_cap::recv_cap;
use crate::server::parse_req::Request;
use crate::server::respond::reply;
use crate::state;

pub fn handle(sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    if body.len() < 4 {
        let _ = reply(sender_pid, MAGIC_NTCP, OP_RECV, E_BAD_LEN, req.request_id, &[], tx);
        return;
    }
    let app_handle = u32::from_le_bytes([body[0], body[1], body[2], body[3]]);

    let sock_handle = match handles::get(app_handle, sender_pid) {
        Some(h) => h,
        None => {
            let _ = reply(sender_pid, MAGIC_NTCP, OP_RECV, E_NO_SOCKET, req.request_id, &[], tx);
            return;
        }
    };

    let mut buf = alloc::vec![0u8; recv_cap(body)];
    let result = state::with_iface(|_iface, sockets, _dev| {
        let sock = sockets.get_mut::<tcp::Socket>(sock_handle);
        sock.recv_slice(&mut buf)
    });

    match result {
        Some(Ok(n)) if n > 0 => {
            let _ = reply(sender_pid, MAGIC_NTCP, OP_RECV, E_OK, req.request_id, &buf[..n], tx);
        }
        _ => {
            let _ = reply(sender_pid, MAGIC_NTCP, OP_RECV, E_RX_EMPTY, req.request_id, &[], tx);
        }
    }
}
