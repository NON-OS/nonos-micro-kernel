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

use core::sync::atomic::Ordering;

use crate::protocol::{E_BAD_LEN, E_OK, E_PERM, OP_SET_CONFIG};
use crate::route::{Route, ROUTES};
use crate::server::authz::authorized;
use crate::server::parse_req::Request;
use crate::server::respond::respond;
use crate::state::IFACE;

// Body: 4 IPv4 + 1 prefix + 4 gateway = 9 bytes. Installs the
// lease and seeds a default route through the gateway.
pub fn handle(sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    if !authorized(sender_pid, b"net.dhcp.client") {
        let _ = respond(sender_pid, OP_SET_CONFIG, E_PERM, req.request_id, 0, tx);
        return;
    }
    if body.len() != 9 {
        let _ = respond(sender_pid, OP_SET_CONFIG, E_BAD_LEN, req.request_id, 0, tx);
        return;
    }
    let mut ipv4 = [0u8; 4];
    ipv4.copy_from_slice(&body[0..4]);
    let prefix = body[4];
    let mut gateway = [0u8; 4];
    gateway.copy_from_slice(&body[5..9]);
    *IFACE.ipv4.lock() = ipv4;
    IFACE.prefix.store(prefix as u16, Ordering::Release);
    *IFACE.gateway.lock() = gateway;
    if gateway != [0; 4] {
        let _ = ROUTES.install(Route { network: [0; 4], prefix: 0, gateway: Some(gateway) });
    }
    let _ = respond(sender_pid, OP_SET_CONFIG, E_OK, req.request_id, 0, tx);
}
