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

use crate::protocol::{E_OK, OP_HEALTHCHECK};
use crate::server::parse_req::Request;
use crate::server::respond::respond;

/// Answer with the gateway the client is bound to, or zeros when it has none.
///
/// Liveness alone reported that the capsule was running, which is the part
/// nobody doubts. Whether it has reached the mixnet is the question being
/// asked, and there was no way to see that from inside the machine.
pub fn handle(pid: u32, req: &Request, tx: &mut [u8]) {
    let off = 20usize;
    match crate::state::TABLE.lock().gateway() {
        Some(gw) => {
            tx[off..off + 4].copy_from_slice(&gw.ip);
            tx[off + 4..off + 6].copy_from_slice(&gw.port.to_le_bytes());
        }
        None => tx[off..off + 6].fill(0),
    }
    respond(pid, OP_HEALTHCHECK, E_OK, req.request_id, 6, tx);
}
