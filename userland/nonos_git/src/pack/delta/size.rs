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
//! The sizes at the head of a delta, and byte access that cannot run off.

use super::super::error::PackError;

pub(super) fn take(delta: &[u8], at: &mut usize) -> Result<u8, PackError> {
    let byte = *delta.get(*at).ok_or(PackError::BadDelta)?;
    *at += 1;
    Ok(byte)
}

/// Base and target size: seven bits per byte, little end first.
pub(super) fn header_size(delta: &[u8], at: &mut usize) -> Result<u64, PackError> {
    let mut value = 0u64;
    let mut shift = 0u32;
    loop {
        let byte = take(delta, at)?;
        value |= u64::from(byte & 0x7F) << shift;
        if byte & 0x80 == 0 {
            return Ok(value);
        }
        shift += 7;
    }
}
