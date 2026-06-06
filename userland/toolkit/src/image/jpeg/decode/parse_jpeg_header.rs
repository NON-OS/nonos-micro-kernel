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
use crate::image::jpeg::marker::{
    is_rst, is_sof_unsupported, read_marker, read_segment_len, M_EOI, M_SOF0, M_SOI,
};
use crate::image::jpeg::sof0::parse_sof0;
use crate::image::types::{DecodeError, ImageSize};

pub fn parse_jpeg_header(input: &[u8]) -> Result<(ImageSize, u8), DecodeError> {
    if input.len() < 2 || input[0] != 0xFF || input[1] != M_SOI {
        return Err(DecodeError::BadMagic);
    }
    let mut pos = 2usize;
    loop {
        let marker = read_marker(input, &mut pos)?;
        match marker {
            M_SOI => continue,
            M_EOI => return Err(DecodeError::Unsupported),
            M_SOF0 => {
                let seg_len = read_segment_len(input, &mut pos)?;
                let frame = parse_sof0(&input[pos..pos + seg_len])?;
                let size = ImageSize::new(frame.width as u32, frame.height as u32)?;
                return Ok((size, frame.num_comps));
            }
            m if is_sof_unsupported(m) => return Err(DecodeError::Unsupported),
            m if is_rst(m) || m == 0x01 => continue,
            _ => {
                let seg_len = read_segment_len(input, &mut pos)?;
                pos += seg_len;
            }
        }
    }
}
