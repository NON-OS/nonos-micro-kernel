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

use super::constants::{
    DNS_MAGIC, NYM_MAGIC, SERVICE_DNS, SERVICE_NYM, SERVICE_SOCKETS, SOCKETS_MAGIC,
};
use super::status::NetStatus;

pub fn probe_network() -> NetStatus {
    let dns_port = super::lookup::lookup(SERVICE_DNS);
    let sockets_port = super::lookup::lookup(SERVICE_SOCKETS);
    let nym_port = super::lookup::lookup(SERVICE_NYM);
    let dns_ok = dns_port != 0 && super::health::health(dns_port, DNS_MAGIC);
    let sockets_ok = sockets_port != 0 && super::health::health(sockets_port, SOCKETS_MAGIC);
    let nym_ok = nym_port != 0 && super::health::health(nym_port, NYM_MAGIC);
    let route_ready = dns_ok && sockets_ok;
    let tcp = if dns_ok && sockets_ok {
        super::probe_rpc_tcp::probe_rpc_tcp(dns_port, sockets_port)
    } else {
        super::probe_rpc_tcp::TcpProbe { resolve: false, socket: false, connect: false }
    };
    let rpc_tcp_ok = tcp.connect;
    let rpc_codec_ok = super::super::rpc::self_check();
    let tls13_ok = super::super::tls13::self_check();
    let tls = if rpc_tcp_ok {
        super::probe_tls_rpc::probe_tls_rpc(dns_port, sockets_port)
    } else {
        super::probe_tls_rpc::TlsProbe::blocked()
    };
    NetStatus {
        dns_ok,
        sockets_ok,
        nym_ok,
        route_ready,
        rpc_resolve_ok: tcp.resolve,
        rpc_socket_ok: tcp.socket,
        rpc_connect_ok: tcp.connect,
        rpc_tcp_ok,
        rpc_codec_ok,
        tls13_ok,
        tls_server_ok: tls.server_hello,
        tls_record_ok: tls.encrypted_record,
        tls_certificate_ok: tls.certificate,
        tls_chain_ok: tls.chain,
        tls_anchor_ok: tls.anchor,
        tls_signature_ok: tls.signature,
        tls_hostname_ok: tls.hostname,
        tls_validity_ok: tls.validity,
        tls_finished_ok: tls.finished,
        tls_client_finished_ok: tls.client_finished,
        rpc_chain_ok: tls.chain_id,
        status: super::probe_status::probe_status(&tls, rpc_tcp_ok, route_ready),
    }
}
