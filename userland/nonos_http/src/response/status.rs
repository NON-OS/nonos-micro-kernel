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
//! The status line.

use super::super::error::HttpError;

/// Read `HTTP/1.x <code> <reason>` and return the code.
pub(super) fn status(line: &[u8]) -> Result<u16, HttpError> {
    if !line.starts_with(b"HTTP/1.") || line.len() < 12 {
        return Err(HttpError::StatusLine);
    }
    let digits = &line[9..12];
    let mut code = 0u16;
    for d in digits {
        if !d.is_ascii_digit() {
            return Err(HttpError::StatusLine);
        }
        code = code * 10 + u16::from(d - b'0');
    }
    Ok(code)
}
