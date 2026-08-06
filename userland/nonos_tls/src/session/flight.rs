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
//! Collecting the server's handshake flight.

extern crate alloc;

use alloc::vec::Vec;
use nonos_libc::{mk_uptime_ms, mk_yield};

use super::settled::settled;
use super::traits::{Io, SessionError};

/// How long a server may stay quiet before it is taken to have stopped
/// sending. The socket layer returns zero when nothing has arrived rather
/// than blocking, so quiet has to be measured.
///
/// A duration rather than a number of reads: an empty read costs microseconds,
/// so counting them gave up long before a server one round trip away could
/// answer at all.
const QUIET_MS: i64 = 4_000;

/// Read until the flight is complete or the server stops sending.
pub(super) fn read_flight<S: Io>(io: &mut S, limit: usize) -> Result<Vec<u8>, SessionError> {
    let mut flight = Vec::new();
    let mut chunk = [0u8; 4096];
    let mut quiet_until = mk_uptime_ms().saturating_add(QUIET_MS);
    loop {
        let n = io.read(&mut chunk)?;
        if n == 0 {
            // A flight holding an application record is complete, and waiting
            // for more would just burn the remaining budget.
            if settled(&flight) {
                return Ok(flight);
            }
            if mk_uptime_ms() >= quiet_until {
                break;
            }
            mk_yield();
            continue;
        }
        quiet_until = mk_uptime_ms().saturating_add(QUIET_MS);
        if flight.len() + n > limit {
            return Err(SessionError::TooLarge);
        }
        flight.extend_from_slice(&chunk[..n]);
        if crate::server_finished_flight_ready(&flight) {
            return Ok(flight);
        }
    }
    if settled(&flight) {
        Ok(flight)
    } else {
        Err(SessionError::Handshake)
    }
}
