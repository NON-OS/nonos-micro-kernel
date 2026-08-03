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

use super::ws;
use crate::protocol::E_GATEWAY_PROTO;
use crate::state::{Gateway, Transport};
use crate::tcp_client;

pub fn connect(tcp_port: u32, mut gateway: Gateway) -> Result<Gateway, u16> {
    let stream = tcp_client::connect(tcp_port, gateway.ip, gateway.port)?;
    gateway.stream = stream;
    if gateway.transport == Transport::WebSocket {
        ws::handshake(tcp_port, gateway).map_err(|_| E_GATEWAY_PROTO)?;
        if gateway.identity != [0u8; 32] {
            gateway.shared_key = super::register::register(tcp_port, &gateway)?;
            crate::state::set_gateway_shared_key(&gateway.shared_key);
            // Without allowance the gateway prices a correct packet and
            // refuses it, which reads as a protocol fault rather than a
            // billing one.
            let _ = super::bandwidth::claim_free_bandwidth(tcp_port, gateway.stream);
        }
    }
    Ok(gateway)
}

pub fn send(tcp_port: u32, gateway: Gateway, payload: &[u8]) -> Result<(), u16> {
    match gateway.transport {
        Transport::RawTcp => tcp_client::send_all(tcp_port, gateway.stream, payload),
        Transport::WebSocket => ws::send_binary(tcp_port, gateway.stream, payload),
    }
}

pub fn recv(tcp_port: u32, gateway: Gateway, out: &mut [u8]) -> Result<usize, u16> {
    match gateway.transport {
        Transport::RawTcp => tcp_client::recv(tcp_port, gateway.stream, out),
        Transport::WebSocket => ws::recv_binary(tcp_port, gateway.stream, out),
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
