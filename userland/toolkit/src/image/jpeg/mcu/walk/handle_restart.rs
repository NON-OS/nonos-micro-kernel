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
use crate::image::jpeg::bits::BitReader;
use crate::image::types::DecodeError;

use super::next_rst_marker::next_rst_marker;

pub fn handle_restart(
    br: &mut BitReader,
    expected: u8,
    pred: &mut [i32; 3],
) -> Result<u8, DecodeError> {
    br.align_to_byte();
    br.flush();
    let mut m = br.marker_hit.take();
    if m.is_none() {
        while br.pos < br.data.len() && br.data[br.pos] != 0xFF {
            br.pos += 1;
        }
        while br.pos < br.data.len() && br.data[br.pos] == 0xFF {
            br.pos += 1;
        }
        if br.pos >= br.data.len() {
            return Err(DecodeError::Truncated);
        }
        let mk = br.data[br.pos];
        br.pos += 1;
        m = Some(mk);
    }
    let marker = m.ok_or(DecodeError::Truncated)?;
    if marker != expected {
        return Err(DecodeError::Unsupported);
    }
    pred[0] = 0;
    pred[1] = 0;
    pred[2] = 0;
    Ok(next_rst_marker(expected))
}
