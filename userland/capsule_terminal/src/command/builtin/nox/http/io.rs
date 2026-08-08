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

use nonos_http::{HttpError, Stream};
use nonos_libc::{mk_uptime_ms, mk_yield};
use nonos_tls::{Io, SessionError};

use crate::mixnet::Wire;

/// The connection seen as a byte stream, whether it runs through the mixnet
/// or straight to the host. TLS reads and writes it directly for `https`;
/// plain `http` drives the same type, so both share one implementation.
pub struct SocketIo {
    pub stream: Wire,
    /// Set once at construction.
    total_deadline: i64,
}

impl SocketIo {
    pub fn new(stream: Wire) -> Self {
        Self { stream, total_deadline: mk_uptime_ms().saturating_add(TOTAL_MS) }
    }
}

impl Io for SocketIo {
    fn write_all(&mut self, data: &[u8]) -> Result<(), SessionError> {
        self.stream.write_all(data).map_err(|_| SessionError::Io)
    }

    fn read(&mut self, into: &mut [u8]) -> Result<usize, SessionError> {
        self.stream.read(into).map_err(|_| SessionError::Io)
    }
}

/// Idle bound: the socket says "nothing yet" with a zero length read while
/// `Stream` defines zero as closed. Bridging the two is this adapter's job.
const QUIET_MS: i64 = 4_000;

/// Total bound. The idle one restarts on every byte, so alone it lets a drip
/// of one byte every few seconds hold the connection forever.
const TOTAL_MS: i64 = 120_000;

impl Stream for SocketIo {
    fn write_all(&mut self, data: &[u8]) -> Result<(), HttpError> {
        self.stream.write_all(data).map_err(|_| HttpError::Io)
    }

    fn read(&mut self, into: &mut [u8]) -> Result<usize, HttpError> {
        let quiet_until = mk_uptime_ms().saturating_add(QUIET_MS);
        loop {
            if mk_uptime_ms() >= self.total_deadline {
                return Err(HttpError::Io);
            }
            let n = self.stream.read(into).map_err(|_| HttpError::Io)?;
            if n > 0 {
                return Ok(n);
            }
            if mk_uptime_ms() >= quiet_until {
                return Ok(0);
            }
            mk_yield();
        }
    }
}
