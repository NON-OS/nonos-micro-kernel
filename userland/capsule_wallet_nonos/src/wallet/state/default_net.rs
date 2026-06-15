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

use crate::wallet::net::NetStatus;

pub fn default_net() -> NetStatus {
    NetStatus {
        dns_ok: false, sockets_ok: false, nym_ok: false, route_ready: false,
        rpc_resolve_ok: false, rpc_socket_ok: false, rpc_connect_ok: false,
        rpc_tcp_ok: false, rpc_codec_ok: false, tls13_ok: false,
        tls_server_ok: false, tls_record_ok: false, tls_certificate_ok: false,
        tls_chain_ok: false, tls_anchor_ok: false, tls_hostname_ok: false,
        tls_signature_ok: false, tls_validity_ok: false, tls_finished_ok: false,
        tls_client_finished_ok: false, rpc_chain_ok: false,
        status: b"net unchecked",
    }
}
