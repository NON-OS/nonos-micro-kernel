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

use super::settled::settled;
use super::traits::{Io, SessionError};

/// How many empty reads in a row are taken to mean the server has finished
/// sending. The socket layer returns zero on a timeout rather than blocking,
/// so quiet has to be counted rather than waited on.
const QUIET_READS: u32 = 40;

/// A ceiling on reads regardless of whether they carry anything, so a peer
/// that dribbles bytes cannot hold the handshake open forever.
const MAX_READS: u32 = 20_000;

/// Read until the flight is complete or the server stops sending.
pub(super) fn read_flight<S: Io>(io: &mut S, limit: usize) -> Result<Vec<u8>, SessionError> {
    let mut flight = Vec::new();
    let mut chunk = [0u8; 4096];
    let mut quiet = 0u32;
    let mut reads = 0u32;
    while quiet < QUIET_READS {
        reads += 1;
        if reads > MAX_READS {
            return Err(SessionError::Handshake);
        }
        let n = io.read(&mut chunk)?;
        if n == 0 {
            quiet += 1;
            // A flight holding an application record is complete, and waiting
            // for more would just burn the remaining budget.
            if settled(&flight) {
                return Ok(flight);
            }
            continue;
        }
        quiet = 0;
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
