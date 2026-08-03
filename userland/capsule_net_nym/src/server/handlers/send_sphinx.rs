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

use crate::gateway_client;
use crate::protocol::{E_CRYPTO, E_NO_ROUTE, E_NO_TCP, E_OK};
use crate::state::Session;

/// Seal a payload as a Sphinx packet and hand it to the gateway.
///
/// Refuses rather than degrades: no route or no gateway key means the message
/// is not sent at all, never sent unprotected.
pub fn send_sphinx(tcp_port: u32, session: &Session, payload: &[u8]) -> u16 {
    let Some(packet) = crate::mixnet::encode_sphinx(&session.dest, &session.dest_id, payload)
    else {
        return E_NO_ROUTE;
    };
    let Ok(frame) =
        gateway_client::make_encrypted_blob(gateway_client::KIND_FORWARD_SPHINX, &packet)
    else {
        return E_CRYPTO;
    };
    gateway_client::send(tcp_port, session.gateway, &frame).map_or(E_NO_TCP, |()| E_OK)
}
