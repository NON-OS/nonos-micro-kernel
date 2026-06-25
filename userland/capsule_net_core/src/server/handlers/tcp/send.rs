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
use crate::protocol::tcp::{E_BAD_LEN, E_NO_SOCKET, E_NOT_CONNECTED, E_OK, MAGIC_NTCP, OP_SEND};
use crate::server::parse_req::Request;
use crate::server::respond::reply;
use crate::state;

pub fn handle(sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    if body.len() < 4 {
        let _ = reply(sender_pid, MAGIC_NTCP, OP_SEND, E_BAD_LEN, req.request_id, &[], tx);
        return;
    }
    let app_handle = u32::from_le_bytes([body[0], body[1], body[2], body[3]]);
    let payload = &body[4..];

    let sock_handle = match handles::get(app_handle, sender_pid) {
        Some(h) => h,
        None => {
            let _ = reply(sender_pid, MAGIC_NTCP, OP_SEND, E_NO_SOCKET, req.request_id, &[], tx);
            return;
        }
    };

    let send_result = state::with_iface(|_iface, sockets, _dev| {
        let sock = sockets.get_mut::<tcp::Socket>(sock_handle);
        sock.send_slice(payload)
    });

    match send_result {
        Some(Ok(n)) => {
            let n = n as u32;
            let _ = reply(sender_pid, MAGIC_NTCP, OP_SEND, E_OK, req.request_id, &n.to_le_bytes(), tx);
        }
        _ => {
            let _ = reply(sender_pid, MAGIC_NTCP, OP_SEND, E_NOT_CONNECTED, req.request_id, &[], tx);
        }
    }
}
