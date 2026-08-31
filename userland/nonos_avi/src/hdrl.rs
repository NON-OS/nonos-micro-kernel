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

use crate::chunk::chunks;
use crate::error::AviError;
use crate::header::{parse_avih, AviHeader};
use crate::stream::{is_video_strh, parse_strf_video, parse_strh_rate, VideoInfo};

pub struct Hdrl {
    pub header: AviHeader,
    pub video: VideoInfo,
    pub stream: u8,
}

pub fn parse_hdrl(d: &[u8]) -> Result<Hdrl, AviError> {
    let mut header = None;
    let mut video = None;
    let mut stream = 0u8;
    let mut seen = 0u8;
    for c in chunks(d) {
        if c.id == *b"avih" {
            header = Some(parse_avih(c.data)?);
            continue;
        }
        if c.id != *b"LIST" || c.data.len() <= 4 || &c.data[0..4] != b"strl" {
            continue;
        }
        let mut rate = (0u32, 0u32);
        for s in chunks(&c.data[4..]) {
            if s.id == *b"strh" && is_video_strh(s.data) {
                rate = parse_strh_rate(s.data)?;
            }
            if s.id == *b"strf" && rate.0 != 0 && video.is_none() {
                video = Some(parse_strf_video(s.data, rate.0, rate.1)?);
                stream = seen;
            }
        }
        seen = seen.saturating_add(1);
    }
    Ok(Hdrl {
        header: header.ok_or(AviError::MissingHeader)?,
        video: video.ok_or(AviError::NoVideoStream)?,
        stream,
    })
}
