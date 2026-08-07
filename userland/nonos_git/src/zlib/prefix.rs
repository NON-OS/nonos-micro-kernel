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
//! Inflating one stream out of a buffer holding more than one.
//!
//! A pack stores its objects as back-to-back zlib streams with no length in
//! front, so reading the next one means knowing exactly how many bytes the
//! last consumed. `decompress` cannot say, since it takes the whole slice as
//! one stream; this returns the byte count alongside the data.

extern crate alloc;

use alloc::vec::Vec;

use super::adler::adler32;
use super::bit_reader::BitReader;
use super::dynamic::dynamic_tables;
use super::error::InflateError;
use super::huffman_block::inflate_block;
use super::stored::inflate_stored;
use super::tables::{fixed_dist, fixed_lit};

/// Inflate the stream at the start of `input`, returning the data and how many
/// bytes of `input` it occupied, trailer included.
pub fn decompress_prefix(input: &[u8]) -> Result<(Vec<u8>, usize), InflateError> {
    if input.len() < 2 {
        return Err(InflateError::Header);
    }
    let (cmf, flg) = (input[0], input[1]);
    if cmf & 0x0F != 8 || flg & 0x20 != 0 || (u16::from(cmf) << 8 | u16::from(flg)) % 31 != 0 {
        return Err(InflateError::Header);
    }

    let mut r = BitReader::new(&input[2..]);
    let mut out: Vec<u8> = Vec::new();
    loop {
        let final_block = r.bit()? == 1;
        match r.bits(2)? {
            0 => inflate_stored(&mut r, &mut out)?,
            1 => inflate_block(&mut r, &mut out, &fixed_lit(), &fixed_dist())?,
            2 => {
                let (lit, dist) = dynamic_tables(&mut r)?;
                inflate_block(&mut r, &mut out, &lit, &dist)?;
            }
            _ => return Err(InflateError::Invalid),
        }
        if final_block {
            break;
        }
    }

    r.align();
    let end = r.byte;
    let trailer = r.data.get(end..end + 4).ok_or(InflateError::Truncated)?;
    let want = u32::from_be_bytes([trailer[0], trailer[1], trailer[2], trailer[3]]);
    if adler32(&out) != want {
        return Err(InflateError::Checksum);
    }
    // Two header bytes, the deflate data, and the four-byte checksum.
    Ok((out, 2 + end + 4))
}
