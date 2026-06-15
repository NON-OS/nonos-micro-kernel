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

pub fn primary(state: &State) -> &'static [u8] {
    if state.net.rpc_chain_ok { b"Ethereum chain 0x1 live" }
    else if state.net.tls_client_finished_ok { b"Client Finished ready" }
    else if state.net.tls_finished_ok { b"Live RPC TLS Finished ok" }
    else if state.net.tls_validity_ok { b"Live RPC cert time valid" }
    else if state.net.tls_hostname_ok { b"Live RPC hostname matched" }
    else if state.net.tls_signature_ok { b"Live RPC CA signatures ok" }
    else if state.net.tls_anchor_ok { b"GTS Root R4 anchor matched" }
    else if state.net.tls_chain_ok { b"Live RPC cert chain received" }
    else if state.net.tls_certificate_ok { b"Live RPC cert message received" }
    else if state.net.tls_record_ok { b"Live RPC TLS record ok" }
    else if state.net.tls_server_ok { b"Live RPC TLS hello ok" }
    else if state.net.tls13_ok && state.net.rpc_codec_ok { b"TLS + RPC codec ready" }
    else if state.net.tls13_ok { b"TLS codec ready" }
    else if state.net.rpc_codec_ok { b"RPC codec ready" }
    else { b"TLS/RPC codec blocked" }
}

pub fn secondary(state: &State) -> &'static [u8] {
    if state.net.rpc_chain_ok { b"Encrypted JSON-RPC read ok" }
    else if state.net.tls_client_finished_ok { b"Application traffic keys derived" }
    else if state.net.tls_finished_ok { b"Server Finished verified" }
    else if state.net.tls_validity_ok { b"Certificate time accepted" }
    else if state.net.tls_hostname_ok { b"Signed certificate matches host" }
    else if state.net.tls_signature_ok { b"Leaf and WE1 signatures verified" }
    else if state.net.tls_anchor_ok { b"Top certificate SPKI trusted" }
    else if state.net.tls_chain_ok { b"CA signature validation pending" }
    else if state.net.tls_certificate_ok { b"Leaf certificate parsed" }
    else if state.net.tls_record_ok { b"Encrypted handshake open" }
    else if state.net.tls_server_ok { b"ServerHello parsed" }
    else if state.net.rpc_connect_ok { b"TCP ready, TLS pending" }
    else if state.net.rpc_socket_ok { b"TCP connect pending" }
    else if state.net.rpc_resolve_ok { b"Socket open pending" }
    else if state.net.route_ready { b"DNS resolve pending" }
    else { b"TCP route pending" }
}
