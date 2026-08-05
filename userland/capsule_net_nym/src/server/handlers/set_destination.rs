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

use crate::protocol::{E_BAD_LEN, E_BUSY, E_NO_SESSION, E_OK, OP_SET_DESTINATION};
use crate::server::handlers::io::u32_at;
use crate::server::parse_req::Request;
use crate::server::respond::respond;
use crate::state::TABLE;

/// Bind a session to a Nym destination: session id, the exit's 32-byte
/// identity, its 32-byte encryption key, then the 32-byte identity of the
/// gateway it is reachable through.
///
/// Both keys are needed and neither substitutes for the other. The identity
/// is where the packet is addressed; the encryption key is what the message
/// inside it is sealed to. The identifier a destination also carries is sent
/// as zeros, which is what a reference client puts there.
pub fn handle(pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    let id = match u32_at(body, 0) {
        Ok(id) => id,
        Err(e) => return respond(pid, OP_SET_DESTINATION, e, req.request_id, 0, tx),
    };
    if body.len() != 4 + 32 + 32 + 32 {
        return respond(pid, OP_SET_DESTINATION, E_BAD_LEN, req.request_id, 0, tx);
    }
    let mut table = TABLE.lock();
    // A pushed mixnet message carries no session id, so delivery is decided by
    // which session holds a destination. Two would make that ambiguous, so the
    // second is refused rather than mis-delivered later.
    if table.sphinx_session_count() > 0 && !table.session_has_dest(pid, id) {
        drop(table);
        return respond(pid, OP_SET_DESTINATION, E_BUSY, req.request_id, 0, tx);
    }
    let errno = table
        .with_mut(pid, id, |s| {
            s.dest.copy_from_slice(&body[4..36]);
            s.dest_encryption.copy_from_slice(&body[36..68]);
            s.dest_gateway.copy_from_slice(&body[68..100]);
            s.dest_id = [0u8; 16];
            E_OK
        })
        .unwrap_or(E_NO_SESSION);
    drop(table);
    respond(pid, OP_SET_DESTINATION, errno, req.request_id, 0, tx);
}
