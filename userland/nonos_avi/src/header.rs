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

use crate::bytes::u32_at;
use crate::error::AviError;

pub struct AviHeader {
    pub micro_sec_per_frame: u32,
    pub total_frames: u32,
    pub width: u32,
    pub height: u32,
}

pub fn parse_avih(d: &[u8]) -> Result<AviHeader, AviError> {
    if d.len() < 56 {
        return Err(AviError::Truncated);
    }
    Ok(AviHeader {
        micro_sec_per_frame: u32_at(d, 0)?,
        total_frames: u32_at(d, 16)?,
        width: u32_at(d, 32)?,
        height: u32_at(d, 36)?,
    })
}
