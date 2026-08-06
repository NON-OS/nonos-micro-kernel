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

//! The inflate driver: zlib header, DEFLATE blocks, Adler-32 trailer.

extern crate alloc;

use alloc::vec::Vec;

use super::adler::adler32;
use super::bit_reader::BitReader;
use super::dynamic::dynamic_tables;
use super::error::InflateError;
use super::huffman_block::inflate_block;
use super::stored::inflate_stored;
use super::tables::{fixed_dist, fixed_lit};

/// Inflate a zlib stream, verifying the header and the trailing Adler-32.
pub fn decompress(input: &[u8]) -> Result<Vec<u8>, InflateError> {
    if input.len() < 2 {
        return Err(InflateError::Header);
    }
    let cmf = input[0];
    let flg = input[1];
    // Deflate method, no preset dictionary, header checksum.
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

    // The four bytes after the deflate data are the Adler-32, big-endian.
    let trailer = r.byte_aligned_tail();
    if trailer.len() < 4 {
        return Err(InflateError::Truncated);
    }
    let want = u32::from_be_bytes([trailer[0], trailer[1], trailer[2], trailer[3]]);
    if adler32(&out) != want {
        return Err(InflateError::Checksum);
    }
    Ok(out)
}
