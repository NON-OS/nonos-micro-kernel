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

use super::put::put;
use super::tables::{DIST_BASE, DIST_EXTRA, LEN_BASE, LEN_EXTRA};

pub fn block(
    bits: &mut BitReader<'_>,
    ll: &Huffman,
    ds: &Huffman,
    out: &mut [u8],
    w: &mut usize,
) -> Result<(), DecodeError> {
    loop {
        let sym = ll.decode(bits)?;
        if sym < 256 {
            put(out, w, sym as u8)?;
        } else if sym == 256 {
            return Ok(());
        } else {
            let si = (sym - 257) as usize;
            if si >= 29 {
                return Err(DecodeError::Unsupported);
            }
            let len = LEN_BASE[si] as usize + bits.read_bits(LEN_EXTRA[si])? as usize;
            let dsym = ds.decode(bits)? as usize;
            if dsym >= 30 {
                return Err(DecodeError::Unsupported);
            }
            let dist = DIST_BASE[dsym] as usize + bits.read_bits(DIST_EXTRA[dsym])? as usize;
            if dist == 0 || dist > *w {
                return Err(DecodeError::Truncated);
            }
            for _ in 0..len {
                let b = out[*w - dist];
                put(out, w, b)?;
            }
        }
    }
}
