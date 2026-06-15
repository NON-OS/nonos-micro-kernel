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

pub fn probe_status(tls: &super::probe_tls_rpc::TlsProbe, rpc_tcp_ok: bool, route_ready: bool) -> &'static [u8] {
    if tls.chain_id { b"rpc chain 0x1" }
    else if tls.client_finished { b"rpc client finish" }
    else if tls.finished { b"rpc tls finished" }
    else if tls.validity { b"rpc cert time" }
    else if tls.hostname { b"rpc host matched" }
    else if tls.signature { b"rpc CA signed" }
    else if tls.anchor { b"rpc CA anchor" }
    else if tls.chain { b"rpc cert chain" }
    else if tls.certificate { b"rpc cert message" }
    else if tls.encrypted_record { b"rpc tls record" }
    else if tls.server_hello { b"rpc tls hello" }
    else if rpc_tcp_ok { b"rpc tcp ready" }
    else if route_ready { b"route ready" }
    else { b"route blocked" }
}
