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
use crate::server::handlers::tcp::connect::ephemeral;
use crate::server::handlers::tcp::connect::types::{ConnectOutcome, Endpoint};
use crate::state;

pub fn open_socket(sender_pid: u32, endpoint: Endpoint) -> ConnectOutcome {
    match state::with_iface(|iface, sockets, _dev| {
        let rx = tcp::SocketBuffer::new(alloc::vec![0u8; 8192]);
        let tx_buf = tcp::SocketBuffer::new(alloc::vec![0u8; 8192]);
        let mut sock = tcp::Socket::new(rx, tx_buf);
        let local = ephemeral::next_ephemeral();
        if sock.connect(iface.context(), (endpoint.remote, endpoint.port), local).is_err() {
            return ConnectOutcome::ConnectFailed;
        }
        let handle = sockets.add(sock);
        match handles::alloc(sender_pid, handle) {
            Some(app_handle) => ConnectOutcome::Ok(app_handle),
            None => {
                sockets.remove(handle);
                ConnectOutcome::TableFull
            }
        }
    }) {
        Some(outcome) => outcome,
        None => ConnectOutcome::TableFull,
    }
}
