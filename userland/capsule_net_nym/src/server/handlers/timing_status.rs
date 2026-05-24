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

use crate::protocol::{E_OK, OP_TIMING_STATUS};
use crate::server::parse_req::Request;
use crate::server::respond::respond;
use crate::state;

const LEN: u32 = 16;

pub fn handle(pid: u32, req: &Request, tx: &mut [u8]) {
    let policy = state::timing_policy();
    let off = 20usize;
    tx[off..off + 2].copy_from_slice(&policy.cover_burst.to_le_bytes());
    tx[off + 2..off + 4].copy_from_slice(&policy.delay_jitter_ms.to_le_bytes());
    tx[off + 4..off + 8].fill(0);
    tx[off + 8..off + 16].copy_from_slice(&state::next_cover_ms().to_le_bytes());
    respond(pid, OP_TIMING_STATUS, E_OK, req.request_id, LEN, tx);
}
