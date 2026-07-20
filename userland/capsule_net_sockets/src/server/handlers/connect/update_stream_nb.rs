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

use crate::clients::tcp;
use crate::protocol::{E_NO_TRANSPORT, E_OK, OP_CONNECT_NB};
use crate::server::parse_req::Request;
use crate::server::respond::respond;
use crate::sockets::{Kind, SocketKey};
use crate::state;

use super::install_transport;

// Non-blocking stream connect: initiate the transport handshake and install it
// immediately, then reply without waiting for establishment. The three-way
// completes in the background (net.core drives its interface from device RX);
// the caller learns the socket is connected by seeing it become writable via
// OP_POLL. Blocking here would pin the single-threaded service and stall a
// reactor-driven client's whole runtime for the round-trip time.
pub fn update_stream_nb(
    pid: u32,
    req: &Request,
    key: SocketKey,
    ip: [u8; 4],
    port: u16,
    tx: &mut [u8],
) {
    let transport = match tcp::connect(state::tcp(), ip, port) {
        Ok(h) => h,
        Err(_) => {
            respond(pid, OP_CONNECT_NB, E_NO_TRANSPORT, req.request_id, 0, tx);
            return;
        }
    };
    let errno = install_transport::install_transport(key, Kind::Stream, ip, port, transport);
    if errno != E_OK {
        let _ = tcp::close(state::tcp(), transport);
    }
    respond(pid, OP_CONNECT_NB, errno, req.request_id, 0, tx);
}
