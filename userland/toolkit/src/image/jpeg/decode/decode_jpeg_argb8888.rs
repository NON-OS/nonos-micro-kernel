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
    is_app, is_rst, is_sof_unsupported, read_marker, M_COM, M_DHT, M_DQT, M_DRI, M_EOI, M_SOF0,
    M_SOI, M_SOS,
};
use crate::image::types::{DecodeError, ImageSize};

use super::handle_dht::handle_dht;
use super::handle_dqt::handle_dqt;
use super::handle_dri::handle_dri;
use super::handle_sof0::handle_sof0;
use super::handle_sos::handle_sos;
use super::skip_segment::skip_segment;
use super::state::DecodeState;

pub fn decode_jpeg_argb8888(input: &[u8], out: &mut [u32]) -> Result<ImageSize, DecodeError> {
    if input.len() < 2 || input[0] != 0xFF || input[1] != M_SOI {
        return Err(DecodeError::BadMagic);
    }
    let mut state = DecodeState::new();
    loop {
        let marker = read_marker(input, &mut state.pos)?;
        match marker {
            M_SOI => continue,
            M_EOI => return Err(DecodeError::Truncated),
            M_SOF0 => handle_sof0(input, out, &mut state)?,
            m if is_sof_unsupported(m) => return Err(DecodeError::Unsupported),
            M_DQT => handle_dqt(input, &mut state)?,
            M_DHT => handle_dht(input, &mut state)?,
            M_DRI => handle_dri(input, &mut state)?,
            M_SOS => return handle_sos(input, out, &mut state),
            M_COM => skip_segment(input, &mut state.pos)?,
            m if is_app(m) => skip_segment(input, &mut state.pos)?,
            m if is_rst(m) || m == 0x01 => continue,
            _ => skip_segment(input, &mut state.pos)?,
        }
    }
}
