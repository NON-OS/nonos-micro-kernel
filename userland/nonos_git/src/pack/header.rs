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
//! The pack header: magic, version, object count.

use super::error::PackError;

/// Bytes before the first object.
pub(super) const HEADER_LEN: usize = 12;

/// Parse the 12-byte header, returning how many objects follow.
pub(super) fn parse(data: &[u8]) -> Result<u32, PackError> {
    if data.len() < HEADER_LEN + 20 {
        return Err(PackError::Truncated);
    }
    if &data[..4] != b"PACK" {
        return Err(PackError::Magic);
    }
    let version = u32::from_be_bytes([data[4], data[5], data[6], data[7]]);
    // Version 3 differs only in ways that do not reach this reader.
    if version != 2 && version != 3 {
        return Err(PackError::Version(version));
    }
    Ok(u32::from_be_bytes([data[8], data[9], data[10], data[11]]))
}
