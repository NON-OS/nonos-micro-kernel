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
//! One request over one connection.

extern crate alloc;

use alloc::vec::Vec;

use super::flight::read_flight;
use super::response::read_response;
use super::traits::{Io, SessionError};

/// Bound on the handshake flight, so a server cannot make a caller allocate
/// without limit before anything has been verified.
const MAX_FLIGHT: usize = 128 * 1024;

/// Handshake with `host`, send `request`, and return the decrypted response.
///
/// One request per connection. Keeping a session open would save a handshake,
/// but it means carrying the plaintext offset across requests, and drift
/// there hands the next caller somebody else's bytes.
pub fn exchange<S: Io>(
    io: &mut S,
    host: &str,
    request: &[u8],
    now: u64,
    limit: usize,
) -> Result<Vec<u8>, SessionError> {
    let cf = crate::client_flight(host.as_bytes()).ok_or(SessionError::Init)?;
    io.write_all(&cf.record)?;

    let flight = read_flight(io, MAX_FLIGHT)?;

    // The certificate chain is checked here, inside application_write. A
    // failure has to stop the request: sending it anyway would hand the
    // payload to whoever answered.
    let out = crate::application_write(&cf, &flight, request, host.as_bytes(), now)
        .ok_or(SessionError::Certificate)?;

    // Derive the server keys once, from the same verified handshake, so the
    // response decrypts without walking the chain again per record.
    let app = crate::server_complete(&cf, &flight, host.as_bytes(), now).map(|sc| sc.app);
    io.write_all(&out)?;

    let buf = read_response(io, limit)?;
    match app {
        Some(app) => Ok(crate::application_plaintext_cached(&app, &buf)),
        None => crate::application_plaintext(&cf, &flight, &buf, host.as_bytes(), now)
            .ok_or(SessionError::Certificate),
    }
}
