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
//! One line of the advertisement.

extern crate alloc;

use alloc::string::String;

use crate::oid::ObjectId;

use super::super::error::WireError;
use super::remote_ref::RemoteRef;

/// `<40 hex> <name>`, with capabilities after a NUL on the first line only.
pub(super) fn parse_ref(line: &[u8]) -> Result<RemoteRef, WireError> {
    let line = match line.iter().position(|b| *b == 0) {
        Some(nul) => &line[..nul],
        None => line,
    };
    let line = line.strip_suffix(b"\n").unwrap_or(line);
    if line.len() < 42 || line[40] != b' ' {
        return Err(WireError::RefLine);
    }
    let hex = core::str::from_utf8(&line[..40]).map_err(|_| WireError::RefLine)?;
    let id = ObjectId::from_hex(hex).ok_or(WireError::RefLine)?;
    let name = core::str::from_utf8(&line[41..]).map_err(|_| WireError::RefLine)?;
    Ok(RemoteRef { id, name: String::from(name) })
}
