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

use crate::protocol::{E_BAD_LEN, E_OK, E_PERM, E_TABLE_FULL, OP_ROUTE_ADD};
use crate::route::{Route, ROUTES};
use crate::server::authz::admin;
use crate::server::parse_req::Request;
use crate::server::respond::respond;

// Body: 4 network + 1 prefix + 4 gateway = 9 bytes. A gateway of
// [0;4] means "on-link". Route metric is not on the wire today —
// longest-prefix-match alone selects the route.
pub fn handle(sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    if !admin(sender_pid) {
        let _ = respond(sender_pid, OP_ROUTE_ADD, E_PERM, req.request_id, 0, tx);
        return;
    }
    if body.len() != 9 {
        let _ = respond(sender_pid, OP_ROUTE_ADD, E_BAD_LEN, req.request_id, 0, tx);
        return;
    }
    let mut network = [0u8; 4];
    network.copy_from_slice(&body[0..4]);
    let prefix = body[4];
    let mut gateway = [0u8; 4];
    gateway.copy_from_slice(&body[5..9]);
    let route =
        Route { network, prefix, gateway: if gateway == [0; 4] { None } else { Some(gateway) } };
    let errno = if ROUTES.install(route).is_ok() { E_OK } else { E_TABLE_FULL };
    let _ = respond(sender_pid, OP_ROUTE_ADD, errno, req.request_id, 0, tx);
}
