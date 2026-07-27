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

use super::constants::ETH_RPC_HOST;
#[derive(Clone, Copy)]
pub struct TlsProbe {
    pub server_hello: bool,
    pub encrypted_record: bool,
    pub certificate: bool,
    pub chain: bool,
    pub anchor: bool,
    pub signature: bool,
    pub hostname: bool,
    pub validity: bool,
    pub finished: bool,
    pub client_finished: bool,
    pub chain_id: bool,
}

impl TlsProbe {
    pub fn blocked() -> Self {
        Self {
            server_hello: false,
            encrypted_record: false,
            certificate: false,
            chain: false,
            anchor: false,
            signature: false,
            hostname: false,
            validity: false,
            finished: false,
            client_finished: false,
            chain_id: false,
        }
    }
}

pub fn probe_tls_rpc(dns_port: u32, sockets_port: u32) -> TlsProbe {
    let Ok(ip) = super::resolve_eth::resolve_eth(dns_port) else { return TlsProbe::blocked() };
    let Ok(handle) = super::socket_open::socket_open(sockets_port) else {
        return TlsProbe::blocked();
    };
    let ok = connect_and_probe(sockets_port, handle, ip);
    let _ = super::socket_close::socket_close(sockets_port, handle);
    ok
}

fn connect_and_probe(sockets_port: u32, handle: u32, ip: [u8; 4]) -> TlsProbe {
    if super::socket_connect::socket_connect(sockets_port, handle, ip, 443).is_err() {
        return TlsProbe::blocked();
    }
    let Some(flight) = super::super::tls13::client_flight(ETH_RPC_HOST) else {
        return TlsProbe::blocked();
    };
    if super::socket_send::socket_send(sockets_port, handle, &flight.record).is_err() {
        return TlsProbe::blocked();
    }
    let Ok(rx) = super::read_tls_flight::read_tls_flight(sockets_port, handle) else {
        return TlsProbe::blocked();
    };
    let client_finished = super::super::tls13::client_finished_flight(&flight, &rx);
    let chain_id = client_finished
        && super::probe_chain_id::probe_chain_id(sockets_port, handle, &flight, &rx);
    let validity = super::rtc_stamp::rtc_stamp()
        .is_some_and(|now| super::super::tls13::server_validity_flight(&flight, &rx, now));
    TlsProbe {
        server_hello: super::super::tls13::server_first_flight(&flight, &rx),
        encrypted_record: super::super::tls13::server_encrypted_flight(&flight, &rx),
        certificate: super::super::tls13::server_certificate_flight(&flight, &rx),
        chain: super::super::tls13::server_chain_flight(&flight, &rx),
        anchor: super::super::tls13::server_anchor_flight(&flight, &rx),
        signature: super::super::tls13::server_signature_flight(&flight, &rx),
        hostname: super::super::tls13::server_hostname_flight(&flight, &rx, ETH_RPC_HOST),
        validity,
        finished: super::super::tls13::server_finished_flight(&flight, &rx),
        client_finished,
        chain_id,
    }
}
