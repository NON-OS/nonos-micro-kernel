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

/// What a remote can make the terminal allocate for one request.
///
/// A depth-1 clone of this kernel is a 33 MB pack, so 16 MB refused it before
/// an object was read. Indexing that pack peaks near four times its size,
/// dominated by its largest single object rather than by how many it holds,
/// which is affordable against the memory a capsule has. 64 MB covers the
/// repositories this is meant for and still refuses a remote that decides to
/// send without end.
const MAX_RESPONSE: usize = 64 * 1024 * 1024;

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
