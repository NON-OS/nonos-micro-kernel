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

use nonos_libc::{mk_time_millis, mk_yield};

use crate::clients::{tcp, udp};
use crate::protocol::{E_NO_HANDLE, E_OK, OP_POLL, POLLIN, POLLOUT};
use crate::server::handlers::io::{u16_at, u32_at};
use crate::server::parse_req::Request;
use crate::server::respond::respond;
use crate::sockets::{stash, Kind, Socket, SocketKey, SOCKETS};
use crate::state;

pub fn handle(pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    let (handle, events, timeout_ms) = match parse(body) {
        Ok(v) => v,
        Err(e) => return reply_err(pid, req, e, tx),
    };
    let Some(sock) = SOCKETS.with(SocketKey { pid, handle }, |s| *s) else {
        return reply_err(pid, req, E_NO_HANDLE, tx);
    };
    let deadline = mk_time_millis().saturating_add(timeout_ms as i64);
    loop {
        let revents = poll_once(pid, handle, sock, events);
        if revents != 0 || !(timeout_ms > 0 && mk_time_millis() < deadline) {
            tx[20..22].copy_from_slice(&revents.to_le_bytes());
            let _ = respond(pid, OP_POLL, E_OK, req.request_id, 2, tx);
            return;
        }
        mk_yield();
    }
}

fn parse(body: &[u8]) -> Result<(u32, u16, u32), u16> {
    Ok((u32_at(body, 0)?, u16_at(body, 4)?, u32_at(body, 8)?))
}

fn poll_once(pid: u32, handle: u32, sock: Socket, events: u16) -> u16 {
    let mut revents = 0;
    if events & POLLIN != 0 && pollin_ready(pid, handle, sock) {
        revents |= POLLIN;
    }
    if events & POLLOUT != 0 && pollout_ready(sock) {
        revents |= POLLOUT;
    }
    revents
}

fn pollin_ready(pid: u32, handle: u32, sock: Socket) -> bool {
    if stash::has(pid, handle) {
        return true;
    }
    let mut buf = [0u8; 2048];
    let got = match sock.kind {
        Kind::Stream if sock.transport_handle != 0 => {
            tcp::recv(state::tcp(), sock.transport_handle, &mut buf)
        }
        Kind::Datagram => match sock.local {
            Some(local) => udp::recv(state::udp(), local.port, &mut buf),
            None => return false,
        },
        _ => return false,
    };
    match got {
        Ok(n) if n > 0 => {
            stash::put(pid, handle, &buf[..n]);
            true
        }
        _ => false,
    }
}

fn pollout_ready(sock: Socket) -> bool {
    match sock.kind {
        Kind::Stream if sock.transport_handle != 0 => tcp::state(state::tcp(), sock.transport_handle)
            .map(|s| s == tcp::ESTABLISHED)
            .unwrap_or(false),
        Kind::Datagram => true,
        _ => false,
    }
}

fn reply_err(pid: u32, req: &Request, errno: u16, tx: &mut [u8]) {
    let _ = respond(pid, OP_POLL, errno, req.request_id, 0, tx);
}
