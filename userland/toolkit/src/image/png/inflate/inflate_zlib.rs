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

use super::block::block;
use super::dynamic::dynamic;
use super::fixed_litlen::fixed_litlen;
use super::stored::stored;

pub fn inflate_zlib(src: &[u8], out: &mut [u8]) -> Result<usize, DecodeError> {
    if src.len() < 2 {
        return Err(DecodeError::Truncated);
    }
    let mut bits = BitReader::new(&src[2..]);
    let mut w = 0usize;
    loop {
        let final_block = bits.read_bits(1)?;
        let btype = bits.read_bits(2)?;
        match btype {
            0 => stored(&mut bits, out, &mut w)?,
            1 => {
                let ll = fixed_litlen()?;
                let ds = Huffman::from_lengths(&[5u8; 30])?;
                block(&mut bits, &ll, &ds, out, &mut w)?;
            }
            2 => {
                let (ll, ds) = dynamic(&mut bits)?;
                block(&mut bits, &ll, &ds, out, &mut w)?;
            }
            _ => return Err(DecodeError::Unsupported),
        }
        if final_block != 0 {
            return Ok(w);
        }
    }
}
