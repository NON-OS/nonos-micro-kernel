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

use alloc::vec::Vec;

use super::constants::ETH_RPC_HOST;

pub fn fetch_rpc(dns_port: u32, sockets_port: u32, body: &[u8]) -> Option<Vec<u8>> {
    let ip = super::resolve_eth::resolve_eth(dns_port).ok()?;
    let handle = super::socket_open::socket_open(sockets_port).ok()?;
    let out = fetch_connected(sockets_port, handle, ip, body);
    let _ = super::socket_close::socket_close(sockets_port, handle);
    out
}

fn fetch_connected(sockets_port: u32, handle: u32, ip: [u8; 4], body: &[u8]) -> Option<Vec<u8>> {
    super::socket_connect::socket_connect(sockets_port, handle, ip, 443).ok()?;
    let flight = super::super::tls13::client_flight(ETH_RPC_HOST)?;
    super::socket_send::socket_send(sockets_port, handle, &flight.record).ok()?;
    let server = super::read_tls_flight::read_tls_flight(sockets_port, handle).ok()?;
    let http = super::super::rpc::http_post(ETH_RPC_HOST, body);
    let out = super::super::tls13::application_write(&flight, &server, &http)?;
    super::socket_send::socket_send(sockets_port, handle, &out).ok()?;
    let response = super::read_tls_flight::read_tls_flight(sockets_port, handle).ok()?;
    super::super::tls13::application_plaintext(&flight, &server, &response)
}
