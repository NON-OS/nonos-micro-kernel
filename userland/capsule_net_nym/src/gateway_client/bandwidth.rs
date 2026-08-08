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

/// The control frame that asks for free testnet allowance.
const CLAIM: &str = "{\"type\":\"claimFreeTestnetBandwidth\"}";

/// Ask the gateway for bandwidth allowance.
///
/// Gateways meter the mixnet and refuse a packet they have no credit for,
/// pricing it at its exact size first. Without this a correctly formed packet
/// comes back as `out_of_bandwidth` and looks like a protocol fault.
///
/// The reply is not waited on: the gateway answers asynchronously alongside
/// everything else on the socket, and the send path already handles a refusal.
pub fn claim_free_bandwidth(tcp_port: u32, stream: u32) -> Result<(), u16> {
    ws::send_text(tcp_port, stream, CLAIM.as_bytes()).map_err(|_| E_GATEWAY_PROTO)
}
