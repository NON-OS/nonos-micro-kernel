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

use super::handshake::{run_handshake, Identity, WsWire};
use crate::protocol::E_GATEWAY_PROTO;
use crate::state::Gateway;

/// Gateway protocol version this client speaks.
const PROTOCOL_VERSION: u64 = 3;

/// Run the registration handshake and return the shared key.
pub fn register(tcp_port: u32, gateway: &Gateway) -> Result<[u8; 32], u16> {
    let own = crate::state::client_identity().ok_or(E_GATEWAY_PROTO)?;
    let mut wire = WsWire { tcp_port, stream: gateway.stream };
    let identity = Identity {
        own_seed: &own.seed,
        own_public: &own.public,
        gateway_public: &gateway.identity,
    };
    run_handshake(&mut wire, &identity, PROTOCOL_VERSION).map_err(|_| E_GATEWAY_PROTO)
}
