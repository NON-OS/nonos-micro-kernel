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

use core::sync::atomic::{AtomicU16, Ordering};
use smoltcp::socket::tcp;
use smoltcp::wire::{IpAddress, Ipv4Address};

use crate::handles;
use crate::protocol::tcp::{E_BAD_LEN, E_NO_SOCKET, E_NOT_CONNECTED, E_OK, MAGIC_NTCP, OP_CONNECT};
use crate::server::parse_req::Request;
use crate::server::respond::reply;
use crate::state;

const EPHEMERAL_BASE: u16 = 49152;
const EPHEMERAL_TOP: u16 = u16::MAX;
static EPHEMERAL: AtomicU16 = AtomicU16::new(EPHEMERAL_BASE);

fn next_ephemeral() -> u16 {
    let p = EPHEMERAL.fetch_add(1, Ordering::Relaxed);
    let range = EPHEMERAL_TOP - EPHEMERAL_BASE + 1;
    EPHEMERAL_BASE + p.wrapping_sub(EPHEMERAL_BASE) % range
}

enum ConnectOutcome {
    Ok(u32),
    ConnectFailed,
    TableFull,
}

pub fn handle(sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    if body.len() < 6 {
        let _ = reply(sender_pid, MAGIC_NTCP, OP_CONNECT, E_BAD_LEN, req.request_id, &[], tx);
        return;
    }
    let ip = [body[0], body[1], body[2], body[3]];
    let port = u16::from_le_bytes([body[4], body[5]]);
    let remote = IpAddress::Ipv4(Ipv4Address(ip));
    let local_port = next_ephemeral();

    let outcome = state::with_iface(|iface, sockets, _dev| {
        let rx = tcp::SocketBuffer::new(alloc::vec![0u8; 8192]);
        let tx_buf = tcp::SocketBuffer::new(alloc::vec![0u8; 8192]);
        let mut sock = tcp::Socket::new(rx, tx_buf);
        if sock.connect(iface.context(), (remote, port), local_port).is_err() {
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
    });

    let (errno, payload): (u16, &[u8]) = match outcome.unwrap_or(ConnectOutcome::TableFull) {
        ConnectOutcome::Ok(app_handle) => {
            let _ = reply(
                sender_pid, MAGIC_NTCP, OP_CONNECT, E_OK,
                req.request_id, &app_handle.to_le_bytes(), tx,
            );
            return;
        }
        ConnectOutcome::ConnectFailed => (E_NOT_CONNECTED, &[]),
        ConnectOutcome::TableFull => (E_NO_SOCKET, &[]),
    };
    let _ = reply(sender_pid, MAGIC_NTCP, OP_CONNECT, errno, req.request_id, payload, tx);
}
