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

/// Take a connected socket up to a registered mixnet session.
///
/// The upgrade request is the first thing written, and `net.tcp` refuses a
/// write while the socket is still SYN-SENT, so the handshake has to be up
/// before the request goes out.
pub fn establish(tcp_port: u32, gateway: &mut Gateway) -> Result<(), u16> {
    tcp_client::wait_established(tcp_port, gateway.stream)
        .inspect_err(|e| super::trace::fail(b"connect", *e))?;
    if gateway.transport != Transport::WebSocket {
        return Ok(());
    }
    ws::handshake(tcp_port, *gateway).map_err(|e| {
        super::trace::fail(b"upgrade", e);
        E_GATEWAY_PROTO
    })?;
    if gateway.identity == [0u8; 32] {
        return Ok(());
    }
    gateway.shared_key = super::register::register(tcp_port, gateway)
        .inspect_err(|e| super::trace::fail(b"register", *e))?;
    crate::state::set_gateway_shared_key(&gateway.shared_key);
    // Without allowance the gateway prices a correct packet and refuses it,
    // which reads as a protocol fault rather than a billing one.
    let _ = super::bandwidth::claim_free_bandwidth(tcp_port, gateway.stream);
    Ok(())
}
