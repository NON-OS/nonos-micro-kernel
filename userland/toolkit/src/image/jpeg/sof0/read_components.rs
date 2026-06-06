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

pub fn read_components(seg: &[u8], nf: u8, frame: &mut FrameHeader) -> Result<(), DecodeError> {
    let mut h_max = 0u8;
    let mut v_max = 0u8;
    let mut i = 0usize;
    while i < nf as usize {
        let base = 6 + i * 3;
        let hv = seg[base + 1];
        let h = (hv >> 4) & 0x0F;
        let v = hv & 0x0F;
        if h == 0 || v == 0 || h > 4 || v > 4 || seg[base + 2] > 3 {
            return Err(DecodeError::Unsupported);
        }
        frame.comps[i].id = seg[base];
        frame.comps[i].h = h;
        frame.comps[i].v = v;
        frame.comps[i].tq = seg[base + 2];
        h_max = h_max.max(h);
        v_max = v_max.max(v);
        i += 1;
    }
    frame.h_max = h_max;
    frame.v_max = v_max;
    Ok(())
}
