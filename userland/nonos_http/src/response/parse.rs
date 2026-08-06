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
//! Turning received bytes into a response.

extern crate alloc;

use alloc::vec::Vec;

use super::super::error::HttpError;
use super::chunk::decode;
use super::headers::headers;
use super::status::status;
use super::types::Response;

/// Parse a complete response.
///
/// The body length comes from the headers rather than from how much arrived:
/// a short read has to be an error, or a truncated pack would be handed on as
/// though it were whole.
pub fn parse_response(raw: &[u8]) -> Result<Response, HttpError> {
    let split = find_blank_line(raw).ok_or(HttpError::Incomplete)?;
    let head = &raw[..split];
    let body = &raw[split + 4..];

    let mut lines = head.split(|b| *b == b'\n');
    let first = lines.next().ok_or(HttpError::StatusLine)?;
    let code = status(first.strip_suffix(b"\r").unwrap_or(first))?;
    let rest_at = first.len() + 1;
    let fields = headers(head.get(rest_at..).unwrap_or(&[]))?;

    let chunked =
        fields.iter().any(|(n, v)| n == "transfer-encoding" && v.eq_ignore_ascii_case("chunked"));
    let body = if chunked {
        decode(body)?
    } else {
        match fields.iter().find(|(n, _)| n == "content-length") {
            Some((_, v)) => {
                let want: usize = v.parse().map_err(|_| HttpError::Body)?;
                if body.len() < want {
                    return Err(HttpError::Body);
                }
                body[..want].to_vec()
            }
            // No length and no chunking means the body runs to the close,
            // which is what Connection: close asks for.
            None => Vec::from(body),
        }
    };

    Ok(Response { status: code, headers: fields, body })
}

fn find_blank_line(raw: &[u8]) -> Option<usize> {
    raw.windows(4).position(|w| w == b"\r\n\r\n")
}
