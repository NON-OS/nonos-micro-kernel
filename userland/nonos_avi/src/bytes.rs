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

use crate::error::AviError;

pub fn u32_at(b: &[u8], o: usize) -> Result<u32, AviError> {
    let e = o.checked_add(4).ok_or(AviError::Truncated)?;
    if e > b.len() {
        return Err(AviError::Truncated);
    }
    Ok(u32::from_le_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]]))
}

pub fn u16_at(b: &[u8], o: usize) -> Result<u16, AviError> {
    let e = o.checked_add(2).ok_or(AviError::Truncated)?;
    if e > b.len() {
        return Err(AviError::Truncated);
    }
    Ok(u16::from_le_bytes([b[o], b[o + 1]]))
}

pub fn fourcc_at(b: &[u8], o: usize) -> Result<[u8; 4], AviError> {
    let e = o.checked_add(4).ok_or(AviError::Truncated)?;
    if e > b.len() {
        return Err(AviError::Truncated);
    }
    Ok([b[o], b[o + 1], b[o + 2], b[o + 3]])
}
