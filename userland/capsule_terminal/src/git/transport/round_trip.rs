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
//! One connection, one request, one response.

extern crate alloc;

use alloc::vec::Vec;

use nonos_git::TransportError;
use nonos_http::parse_response;
use nonos_socket::TcpStream;
use nonos_tls::exchange;

use super::https::Https;
use super::io::SocketIo;

/// A packfile for a small repository runs to a few megabytes. This bounds
/// what a remote can make the terminal allocate for one request.
const MAX_RESPONSE: usize = 16 * 1024 * 1024;

/// Connect, handshake, send, read, and hand back the body.
///
/// A status other than 200 is an error rather than a body, because git's own
/// error pages are valid HTTP and would otherwise be parsed as a pack.
pub(super) fn round_trip(https: &mut Https, request: Vec<u8>) -> Result<Vec<u8>, TransportError> {
    let stream =
        TcpStream::connect(&https.remote.host, 443).map_err(|_| TransportError::Unreachable)?;
    let mut io = SocketIo { stream };
    let raw = exchange(&mut io, &https.remote.host, &request, https.now, MAX_RESPONSE)
        .map_err(|_| TransportError::Closed)?;
    let response = parse_response(&raw).map_err(|_| TransportError::Malformed)?;
    if response.status != 200 {
        return Err(TransportError::Status(response.status));
    }
    Ok(response.body)
}
