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
//! Header lines.

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use super::super::error::HttpError;

/// Split one header line into a lowercased name and a trimmed value.
///
/// Names are compared case insensitively by the spec, and lowercasing once
/// here is what lets every lookup afterwards be a plain comparison.
pub(super) fn header(line: &[u8]) -> Result<(String, String), HttpError> {
    let colon = line.iter().position(|b| *b == b':').ok_or(HttpError::Header)?;
    let (name, rest) = line.split_at(colon);
    if name.is_empty() {
        return Err(HttpError::Header);
    }
    let mut lower = String::with_capacity(name.len());
    for b in name {
        lower.push(b.to_ascii_lowercase() as char);
    }
    let value = core::str::from_utf8(&rest[1..]).map_err(|_| HttpError::Header)?;
    Ok((lower, String::from(value.trim())))
}

/// Every header line up to the blank line, and where the body starts.
pub(super) fn headers(head: &[u8]) -> Result<Vec<(String, String)>, HttpError> {
    let mut out = Vec::new();
    for line in head.split(|b| *b == b'\n') {
        let line = line.strip_suffix(b"\r").unwrap_or(line);
        if line.is_empty() {
            continue;
        }
        out.push(header(line)?);
    }
    Ok(out)
}
