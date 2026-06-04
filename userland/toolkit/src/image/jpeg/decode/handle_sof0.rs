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
use crate::image::jpeg::marker::read_segment_len;
use crate::image::jpeg::sof0::parse_sof0;
use crate::image::types::{DecodeError, ImageSize};

use super::state::DecodeState;

pub fn handle_sof0(input: &[u8], out: &[u32], state: &mut DecodeState) -> Result<(), DecodeError> {
    let seg_len = read_segment_len(input, &mut state.pos)?;
    let seg = &input[state.pos..state.pos + seg_len];
    let frame = parse_sof0(seg)?;
    let size = ImageSize::new(frame.width as u32, frame.height as u32)?;
    if (size.pixel_count() as usize) > out.len() {
        return Err(DecodeError::OutputTooSmall);
    }
    state.frame_opt = Some(frame);
    state.pos += seg_len;
    Ok(())
}
