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

use crate::bytes::{fourcc_at, u32_at};
use crate::error::AviError;

pub struct VideoInfo {
    pub rate: u32,
    pub scale: u32,
    pub codec: [u8; 4],
    pub width: u32,
    pub height: u32,
}

pub fn is_video_strh(d: &[u8]) -> bool {
    matches!(fourcc_at(d, 0), Ok(t) if t == *b"vids")
}

pub fn parse_strh_rate(d: &[u8]) -> Result<(u32, u32), AviError> {
    if d.len() < 32 {
        return Err(AviError::Truncated);
    }
    Ok((u32_at(d, 24)?, u32_at(d, 20)?))
}

pub fn parse_strf_video(d: &[u8], rate: u32, scale: u32) -> Result<VideoInfo, AviError> {
    if d.len() < 40 {
        return Err(AviError::Truncated);
    }
    let height = u32_at(d, 8)? as i32;
    Ok(VideoInfo {
        rate,
        scale,
        codec: fourcc_at(d, 16)?,
        width: u32_at(d, 4)?,
        height: height.unsigned_abs(),
    })
}
