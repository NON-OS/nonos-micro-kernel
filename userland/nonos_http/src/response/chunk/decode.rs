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
//! Reassembling a chunked body.

extern crate alloc;

use alloc::vec::Vec;

use super::super::super::error::HttpError;
use super::size::{hex, line_end};

/// Each chunk is a hex length, CRLF, that many bytes, CRLF. A zero length
/// ends the body, and anything after it is trailers this does not need.
pub(in crate::response) fn decode(body: &[u8]) -> Result<Vec<u8>, HttpError> {
    let mut out = Vec::with_capacity(body.len());
    let mut at = 0usize;
    loop {
        let end_of_size = line_end(body, at)?;
        let size = hex(&body[at..end_of_size])?;
        at = end_of_size + 2;
        if size == 0 {
            return Ok(out);
        }
        let end = at.checked_add(size).ok_or(HttpError::Chunk)?;
        if end > body.len() {
            return Err(HttpError::Body);
        }
        out.extend_from_slice(&body[at..end]);
        // Every chunk is followed by its own CRLF.
        at = end + 2;
        if at > body.len() {
            return Err(HttpError::Body);
        }
    }
}
