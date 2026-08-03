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
//! Reading one chunk size line.

use super::super::super::error::HttpError;

/// Where the CRLF after `from` is.
pub(super) fn line_end(body: &[u8], from: usize) -> Result<usize, HttpError> {
    let mut at = from;
    while at + 1 < body.len() {
        if body[at] == b'\r' && body[at + 1] == b'\n' {
            return Ok(at);
        }
        at += 1;
    }
    Err(HttpError::Chunk)
}

/// The size a chunk header states.
pub(super) fn hex(line: &[u8]) -> Result<usize, HttpError> {
    // A chunk size may carry extensions after a semicolon; they are ignored.
    let digits = match line.iter().position(|b| *b == b';') {
        Some(at) => &line[..at],
        None => line,
    };
    if digits.is_empty() {
        return Err(HttpError::Chunk);
    }
    let mut n = 0usize;
    for d in digits {
        let v = match d {
            b'0'..=b'9' => d - b'0',
            b'a'..=b'f' => d - b'a' + 10,
            b'A'..=b'F' => d - b'A' + 10,
            _ => return Err(HttpError::Chunk),
        };
        n = n
            .checked_mul(16)
            .and_then(|n| n.checked_add(usize::from(v)))
            .ok_or(HttpError::Chunk)?;
    }
    Ok(n)
}
