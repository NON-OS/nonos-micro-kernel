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

use nonos_libc::{mk_uptime_ms, mk_yield};

use super::state::{state, CLOSED, ESTABLISHED, SYN_RECEIVED, SYN_SENT};

const E_TIMEOUT: u16 = 6;
const E_REFUSED: u16 = 7;
/// Unexpected states are reported as this plus the state code, so a log line
/// says which state stopped the wait rather than only that one did.
const STATE_BASE: u16 = 100;
const DEADLINE_MS: i64 = 5_000;

/// Block until the three-way handshake completes.
///
/// `connect` returns as soon as the SYN is queued, so a write issued straight
/// after it races the handshake and `net.tcp` rejects it while the socket is
/// still SYN-SENT. A gateway that is merely far away is not a dead gateway,
/// so the wait is bounded by time rather than by attempts.
pub fn wait_established(port: u32, handle: u32) -> Result<(), u16> {
    let deadline = mk_uptime_ms().saturating_add(DEADLINE_MS);
    // net.tcp reports Closed until its interface is next polled, so the first
    // reads after connect say Closed for a socket that is about to open.
    // Closed only means refused once the socket has been seen on its way up.
    let mut opening = false;
    loop {
        match state(port, handle)? {
            ESTABLISHED => return Ok(()),
            SYN_SENT | SYN_RECEIVED => opening = true,
            CLOSED if opening => return Err(E_REFUSED),
            CLOSED => {}
            // Any other state means the peer took the connection somewhere
            // this client cannot use, so stop rather than spin to the
            // deadline. The state itself is carried in the error so the log
            // names which one it was.
            other => return Err(STATE_BASE + other as u16),
        }
        if mk_uptime_ms() >= deadline {
            return Err(E_TIMEOUT);
        }
        mk_yield();
    }
}
