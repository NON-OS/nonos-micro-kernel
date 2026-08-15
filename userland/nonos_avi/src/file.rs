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

use crate::chunk::chunks;
use crate::error::AviError;
use crate::hdrl::parse_hdrl;
use crate::header::AviHeader;
use crate::index::{parse_idx1, FrameRef};
use crate::stream::VideoInfo;

pub struct AviFile {
    pub header: AviHeader,
    pub video: VideoInfo,
    pub index: Vec<FrameRef>,
}

impl AviFile {
    pub fn parse(file: &[u8]) -> Result<AviFile, AviError> {
        if file.len() < 12 {
            return Err(AviError::Truncated);
        }
        if &file[0..4] != b"RIFF" {
            return Err(AviError::NotRiff);
        }
        if &file[8..12] != b"AVI " {
            return Err(AviError::NotAvi);
        }
        let mut hdrl = None;
        let mut movi_data_pos = 0u64;
        let mut idx1 = None;
        for c in chunks(&file[12..]) {
            if c.id == *b"LIST" && c.data.len() > 4 {
                if &c.data[0..4] == b"hdrl" {
                    hdrl = Some(parse_hdrl(&c.data[4..])?);
                } else if &c.data[0..4] == b"movi" {
                    movi_data_pos = offset_in(file, c.data);
                }
            } else if c.id == *b"idx1" {
                idx1 = Some(c.data);
            }
        }
        let hdrl = hdrl.ok_or(AviError::MissingHeader)?;
        let idx1 = idx1.ok_or(AviError::NoIndex)?;
        let index = parse_idx1(file, movi_data_pos, idx1, hdrl.stream)?;
        Ok(AviFile { header: hdrl.header, video: hdrl.video, index })
    }

    pub fn fps_milli(&self) -> u32 {
        if self.video.scale == 0 {
            return 0;
        }
        ((self.video.rate as u64 * 1000) / self.video.scale as u64) as u32
    }
}

fn offset_in(file: &[u8], part: &[u8]) -> u64 {
    (part.as_ptr() as usize - file.as_ptr() as usize) as u64
}
