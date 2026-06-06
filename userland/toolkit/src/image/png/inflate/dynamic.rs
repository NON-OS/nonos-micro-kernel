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
use crate::image::png::huffman::Huffman;
use crate::image::types::DecodeError;

use super::tables::CL_ORDER;

pub fn dynamic(bits: &mut BitReader<'_>) -> Result<(Huffman, Huffman), DecodeError> {
    let hlit = bits.read_bits(5)? as usize + 257;
    let hdist = bits.read_bits(5)? as usize + 1;
    let hclen = bits.read_bits(4)? as usize + 4;
    let mut cl = [0u8; 19];
    for i in 0..hclen {
        cl[CL_ORDER[i]] = bits.read_bits(3)? as u8;
    }
    let clh = Huffman::from_lengths(&cl)?;
    let total = hlit + hdist;
    let mut lens = [0u8; 320];
    let mut i = 0usize;
    while i < total {
        let s = clh.decode(bits)?;
        match s {
            0..=15 => {
                lens[i] = s as u8;
                i += 1;
            }
            16 => {
                if i == 0 {
                    return Err(DecodeError::Unsupported);
                }
                let r = bits.read_bits(2)? as usize + 3;
                let p = lens[i - 1];
                for _ in 0..r {
                    if i >= total {
                        return Err(DecodeError::Unsupported);
                    }
                    lens[i] = p;
                    i += 1;
                }
            }
            17 => i += (bits.read_bits(3)? as usize + 3).min(total - i),
            18 => i += (bits.read_bits(7)? as usize + 11).min(total - i),
            _ => return Err(DecodeError::Unsupported),
        }
    }
    Ok((Huffman::from_lengths(&lens[..hlit])?, Huffman::from_lengths(&lens[hlit..total])?))
}
