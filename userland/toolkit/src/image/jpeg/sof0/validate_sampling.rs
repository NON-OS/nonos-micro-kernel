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

pub fn validate_sampling(frame: &FrameHeader) -> Result<(), DecodeError> {
    if frame.num_comps == 1 {
        if frame.comps[0].h != 1 || frame.comps[0].v != 1 {
            return Err(DecodeError::Unsupported);
        }
        return Ok(());
    }
    let h0 = frame.comps[0].h;
    let v0 = frame.comps[0].v;
    let chroma = frame.comps[1].h == 1
        && frame.comps[1].v == 1
        && frame.comps[2].h == 1
        && frame.comps[2].v == 1;
    let luma = (h0 == 1 && v0 == 1)
        || (h0 == 2 && v0 == 1)
        || (h0 == 1 && v0 == 2)
        || (h0 == 2 && v0 == 2);
    if chroma && luma {
        Ok(())
    } else {
        Err(DecodeError::Unsupported)
    }
}
