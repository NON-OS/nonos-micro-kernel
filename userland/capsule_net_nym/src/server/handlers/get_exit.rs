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

use super::find_exit::find_exit;
use crate::protocol::{E_NO_ROUTE, E_OK, OP_GET_EXIT};
use crate::server::parse_req::Request;
use crate::server::respond::respond;

/// Hand back an exit taken from the directory.
///
/// A client that compiles an exit in trusts whoever built it, and that list
/// ages: an operator who stops running a requester leaves every client that
/// shipped with it unable to reach anything. Asking the network instead means
/// the answer is as current as the directory is.
///
/// The reply is the exit's identity, its encryption key, and the gateway it
/// sits behind, in that order.
pub fn handle(pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    // Callers ask by position so they can walk past one that will not answer.
    let index = if body.len() >= 4 {
        u32::from_le_bytes([body[0], body[1], body[2], body[3]]) as usize
    } else {
        0
    };

    let Some(exit) = find_exit(index) else {
        return respond(pid, OP_GET_EXIT, E_NO_ROUTE, req.request_id, 0, tx);
    };
    if tx.len() < 20 + 96 {
        return respond(pid, OP_GET_EXIT, E_NO_ROUTE, req.request_id, 0, tx);
    }
    tx[20..52].copy_from_slice(&exit.identity);
    tx[52..84].copy_from_slice(&exit.encryption);
    tx[84..116].copy_from_slice(&exit.gateway);
    respond(pid, OP_GET_EXIT, E_OK, req.request_id, 96, tx);
}
