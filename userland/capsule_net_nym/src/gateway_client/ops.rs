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

use super::establish::establish;
use super::ws;
use crate::state::{Gateway, Transport};
use crate::tcp_client;

pub fn connect(tcp_port: u32, mut gateway: Gateway) -> Result<Gateway, u16> {
    let stream = tcp_client::connect(tcp_port, gateway.ip, gateway.port)?;
    gateway.stream = stream;
    match establish(tcp_port, &mut gateway) {
        Ok(()) => Ok(gateway),
        Err(e) => {
            // A candidate that fails partway leaves the socket open. Closing
            // it here is what keeps a run down the bootstrap list from
            // stranding one connection per gateway it tried.
            let _ = tcp_client::close(tcp_port, stream);
            Err(e)
        }
    }
}

pub fn send(tcp_port: u32, gateway: Gateway, payload: &[u8]) -> Result<(), u16> {
    match gateway.transport {
        Transport::RawTcp => tcp_client::send_all(tcp_port, gateway.stream, payload),
        Transport::WebSocket => ws::send_binary(tcp_port, gateway.stream, payload),
    }
}

pub fn recv(
    tcp_port: u32,
    gateway: Gateway,
    out: &mut [u8],
    wait_ms: i64,
) -> Result<ws::Frame, u16> {
    match gateway.transport {
        // A raw link carries the same blobs with no framing of its own, so
        // everything on it is binary by construction.
        Transport::RawTcp => tcp_client::recv(tcp_port, gateway.stream, out)
            .map(|len| ws::Frame { len, text: false }),
        Transport::WebSocket => ws::recv_binary(tcp_port, gateway.stream, out, wait_ms),
    }
}

pub fn close(tcp_port: u32, gateway: Gateway) -> Result<(), u16> {
    let mut frame_err = None;
    if gateway.transport == Transport::WebSocket {
        if let Err(e) = ws::send_close(tcp_port, gateway.stream) {
            frame_err = Some(e);
        }
    }
    tcp_client::close(tcp_port, gateway.stream)?;
    if let Some(e) = frame_err {
        return Err(e);
    }
    Ok(())
}

/// Ping the gateway so an idle link is not closed under us.
pub fn ping(tcp_port: u32, gateway: Gateway) -> Result<(), u16> {
    match gateway.transport {
        Transport::RawTcp => Ok(()),
        Transport::WebSocket => ws::send_ping(tcp_port, gateway.stream),
    }
}
