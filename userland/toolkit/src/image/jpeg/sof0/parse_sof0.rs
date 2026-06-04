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
use crate::image::types::DecodeError;

use super::frame_header::FrameHeader;
use super::read_components::read_components;
use super::validate_sampling::validate_sampling;

pub fn parse_sof0(seg: &[u8]) -> Result<FrameHeader, DecodeError> {
    if seg.len() < 6 {
        return Err(DecodeError::Truncated);
    }
    let precision = seg[0];
    if precision != 8 {
        return Err(DecodeError::Unsupported);
    }
    let height = u16::from_be_bytes([seg[1], seg[2]]);
    let width = u16::from_be_bytes([seg[3], seg[4]]);
    let nf = seg[5];
    if height == 0 || width == 0 {
        return Err(DecodeError::BadDimensions);
    }
    if nf != 1 && nf != 3 {
        return Err(DecodeError::Unsupported);
    }
    if seg.len() < 6 + (nf as usize) * 3 {
        return Err(DecodeError::Truncated);
    }
    let mut frame = FrameHeader::new();
    frame.precision = precision;
    frame.width = width;
    frame.height = height;
    frame.num_comps = nf;
    read_components(seg, nf, &mut frame)?;
    validate_sampling(&frame)?;
    Ok(frame)
}
