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

pub fn probe_chain_id(sockets_port: u32, handle: u32, flight: &super::super::tls13::flight::ClientFlight, rx: &[u8]) -> bool {
    let body = super::super::rpc::request_chain_id(1);
    let http = super::super::rpc::http_post(ETH_RPC_HOST, &body);
    let Some(out) = super::super::tls13::application_write(flight, rx, &http) else { return false };
    if super::socket_send::socket_send(sockets_port, handle, &out).is_err() {
        return false;
    }
    let Ok(resp) = super::read_tls_flight::read_tls_flight(sockets_port, handle) else { return false };
    let Some(plain) = super::super::tls13::application_plaintext(flight, rx, &resp) else { return false };
    plain.windows(14).any(|w| w == b"\"result\":\"0x1\"")
}
