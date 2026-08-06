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
//! The two variable-length integer encodings a pack uses.

use super::error::PackError;

/// The size field in an object header: seven bits per byte, little end first,
/// continuing while the top bit is set. The caller supplies the first chunk
/// and its width, since the object header packs the type into the same byte.
pub(super) fn size(
    data: &[u8],
    at: &mut usize,
    first: u64,
    mut shift: u32,
) -> Result<u64, PackError> {
    let mut value = first;
    let mut more = data.get(*at - 1).copied().ok_or(PackError::Truncated)? & 0x80 != 0;
    while more {
        let byte = *data.get(*at).ok_or(PackError::Truncated)?;
        *at += 1;
        value |= u64::from(byte & 0x7F) << shift;
        shift += 7;
        more = byte & 0x80 != 0;
    }
    Ok(value)
}

/// The negative offset on an ofs-delta. Unlike the size encoding this one adds
/// one per continuation, so the same byte sequence never names two offsets.
pub(super) fn offset(data: &[u8], at: &mut usize) -> Result<u64, PackError> {
    let mut byte = *data.get(*at).ok_or(PackError::Truncated)?;
    *at += 1;
    let mut value = u64::from(byte & 0x7F);
    while byte & 0x80 != 0 {
        byte = *data.get(*at).ok_or(PackError::Truncated)?;
        *at += 1;
        value = ((value + 1) << 7) | u64::from(byte & 0x7F);
    }
    Ok(value)
}
