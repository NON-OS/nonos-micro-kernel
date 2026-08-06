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

use crate::protocol::{E_BAD_LEN, E_OK, E_PERM, OP_SET_IDENTITY};
use crate::server::authz::admin;
use crate::server::parse_req::Request;
use crate::server::respond::respond;
use crate::state::set_client_identity;

/// Install the client identity: 32-byte Ed25519 seed then its 32-byte public
/// key. Admin only, since it decides who this node is to every gateway.
pub fn handle(pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    if !admin(pid) {
        return respond(pid, OP_SET_IDENTITY, E_PERM, req.request_id, 0, tx);
    }
    if body.len() != 64 {
        return respond(pid, OP_SET_IDENTITY, E_BAD_LEN, req.request_id, 0, tx);
    }
    let mut seed = [0u8; 32];
    let mut public = [0u8; 32];
    seed.copy_from_slice(&body[..32]);
    public.copy_from_slice(&body[32..]);
    set_client_identity(&seed, &public);
    respond(pid, OP_SET_IDENTITY, E_OK, req.request_id, 0, tx);
}
