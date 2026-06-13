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

use super::empty_rail::empty_rail;
use super::types::{State, MAX_RAILS};
use crate::wallet::net::NetStatus;

pub fn new_state() -> State {
    State {
        keyring_port: 0,
        owner_pid: 0,
        wallet_id: 0,
        address: [0; 20],
        address_ready: false,
        view: super::types::VIEW_HOME,
        send_focus: super::types::SEND_FIELD_TO,
        send_to_hex: [0; 40],
        send_to_len: 0,
        send_amount_milli_eth: 1,
        send_nonce: 0,
        tx_hash: [0; 32],
        tx_len: 0,
        tx_ready: false,
        tx_kind: b"none",
        proof_count: 0,
        proof_eth_hash: [0; 32],
        proof_eth_len: 0,
        proof_nox_hash: [0; 32],
        proof_nox_len: 0,
        net: NetStatus {
            dns_ok: false,
            sockets_ok: false,
            nym_ok: false,
            route_ready: false,
            rpc_tcp_ok: false,
            rpc_codec_ok: false,
            status: b"net unchecked",
        },
        rails: [empty_rail(); MAX_RAILS],
        rail_count: 0,
        status: b"keyring pending",
    }
}
