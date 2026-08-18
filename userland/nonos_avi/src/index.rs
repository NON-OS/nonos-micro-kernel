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

use alloc::vec::Vec;

use crate::bytes::{fourcc_at, u32_at};
use crate::error::AviError;

pub struct FrameRef {
    pub offset: u64,
    pub len: u32,
}

fn stream_of(id: [u8; 4]) -> Option<u8> {
    let tens = (id[0] as char).to_digit(10)?;
    let ones = (id[1] as char).to_digit(10)?;
    Some((tens * 10 + ones) as u8)
}

fn points_at_chunk(file: &[u8], at: u64, want: [u8; 4]) -> bool {
    let Ok(o) = usize::try_from(at) else {
        return false;
    };
    matches!(fourcc_at(file, o), Ok(t) if t == want)
}

fn base_for(file: &[u8], movi_data_pos: u64, d: &[u8]) -> Result<u64, AviError> {
    let first = u32_at(d, 8)? as u64;
    let id = fourcc_at(d, 0)?;
    let relative = movi_data_pos
        .checked_add(first)
        .map(|a| points_at_chunk(file, a, id))
        .unwrap_or(false);
    if relative {
        return Ok(movi_data_pos);
    }
    if points_at_chunk(file, first, id) {
        return Ok(0);
    }
    Err(AviError::Truncated)
}

pub fn parse_idx1(
    file: &[u8],
    movi_data_pos: u64,
    d: &[u8],
    stream: u8,
) -> Result<Vec<FrameRef>, AviError> {
    let base = base_for(file, movi_data_pos, d)?;
    let mut out = Vec::new();
    out.try_reserve(d.len() / 16)
        .map_err(|_| AviError::TooManyFrames)?;
    for e in d.chunks_exact(16) {
        let id = fourcc_at(e, 0)?;
        if stream_of(id) != Some(stream) {
            continue;
        }
        let chunk_at = base
            .checked_add(u32_at(e, 8)? as u64)
            .ok_or(AviError::Truncated)?;
        let offset = chunk_at.checked_add(8).ok_or(AviError::Truncated)?;
        out.push(FrameRef { offset, len: u32_at(e, 12)? });
    }
    if out.is_empty() {
        return Err(AviError::NoFrames);
    }
    Ok(out)
}
