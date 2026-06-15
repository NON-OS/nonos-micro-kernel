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

use crate::wallet::state::State;

pub fn paint_send_route_label(state: &State) -> &'static [u8] {
    if state.net.rpc_chain_ok {
        b"Ethereum chain 0x1 live"
    } else if state.net.tls_client_finished_ok {
        b"Client Finished ready"
    } else if state.net.tls_finished_ok {
        b"RPC TLS Finished ok"
    } else if state.net.tls_validity_ok {
        b"RPC cert time valid"
    } else if state.net.tls_hostname_ok {
        b"RPC hostname matched"
    } else if state.net.tls_certificate_ok {
        b"RPC cert chain received"
    } else if state.net.tls_record_ok {
        b"RPC TLS record ok"
    } else if state.net.tls_server_ok {
        b"RPC TLS hello ok"
    } else if state.net.rpc_tcp_ok {
        b"TCP ready, TLS pending"
    } else {
        b"TLS pending"
    }
}
