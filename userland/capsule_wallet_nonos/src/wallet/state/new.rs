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

pub fn new_state() -> State {
    State {
        keyring_port: 0,
        owner_pid: 0,
        wallet_id: 0,
        address: [0; 20],
        address_ready: false,
        tx_hash: [0; 32],
        tx_len: 0,
        tx_ready: false,
        tx_kind: b"none",
        proof_count: 0,
        rails: [empty_rail(); MAX_RAILS],
        rail_count: 0,
        status: b"keyring pending",
    }
}
