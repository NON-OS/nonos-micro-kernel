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
use crate::image::png::deflate::BitReader;
use crate::image::types::DecodeError;

use super::put::put;

pub fn stored(bits: &mut BitReader<'_>, out: &mut [u8], w: &mut usize) -> Result<(), DecodeError> {
    bits.align_byte();
    let len = bits.read_bits(16)? as usize;
    let nlen = bits.read_bits(16)?;
    if nlen != (!(len as u16)) {
        return Err(DecodeError::BadMagic);
    }
    for _ in 0..len {
        let b = bits.read_bits(8)? as u8;
        put(out, w, b)?;
    }
    Ok(())
}
