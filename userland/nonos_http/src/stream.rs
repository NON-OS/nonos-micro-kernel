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
//! Doing a request over a stream someone else opened.

extern crate alloc;

use alloc::vec::Vec;

use super::error::HttpError;
use super::request::Request;
use super::response::{parse_response, Response};

/// A byte stream: a TCP socket, a TLS session, or a buffer in a test.
pub trait Stream {
    fn write_all(&mut self, data: &[u8]) -> Result<(), HttpError>;
    /// Read what is available. Zero means the peer closed.
    fn read(&mut self, into: &mut [u8]) -> Result<usize, HttpError>;
}

/// Send `request` and read the response until the stream closes.
///
/// `limit` bounds what a remote can make this allocate. A server that says it
/// will send more than the caller allows is refused before the body is read
/// rather than after.
pub fn fetch<S: Stream>(
    stream: &mut S,
    request: &Request,
    limit: usize,
) -> Result<Response, HttpError> {
    stream.write_all(&request.bytes)?;

    let mut raw = Vec::new();
    let mut buf = [0u8; 4096];
    loop {
        let n = stream.read(&mut buf)?;
        if n == 0 {
            break;
        }
        if raw.len() + n > limit {
            return Err(HttpError::TooLarge);
        }
        raw.extend_from_slice(&buf[..n]);
    }
    parse_response(&raw)
}
