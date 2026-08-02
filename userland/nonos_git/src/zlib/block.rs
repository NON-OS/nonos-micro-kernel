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

//! Decoding one DEFLATE block: stored bytes, or Huffman literal/length and
//! distance symbols until the end-of-block code.

extern crate alloc;

use alloc::vec::Vec;

use super::bit_reader::BitReader;
use super::error::InflateError;
use super::huffman::Huffman;
use super::tables::{DIST_BASE, DIST_EXTRA, LEN_BASE, LEN_EXTRA};

/// A stored block: byte-align, read LEN and its complement, copy LEN bytes.
pub(super) fn inflate_stored(r: &mut BitReader<'_>, out: &mut Vec<u8>) -> Result<(), InflateError> {
    r.align();
    if r.byte + 4 > r.data.len() {
        return Err(InflateError::Truncated);
    }
    let len = u16::from_le_bytes([r.data[r.byte], r.data[r.byte + 1]]);
    let nlen = u16::from_le_bytes([r.data[r.byte + 2], r.data[r.byte + 3]]);
    if len != !nlen {
        return Err(InflateError::Invalid);
    }
    r.byte += 4;
    let end = r.byte + len as usize;
    if end > r.data.len() {
        return Err(InflateError::Truncated);
    }
    out.extend_from_slice(&r.data[r.byte..end]);
    r.byte = end;
    Ok(())
}

/// A compressed block: decode symbols with `lit` and `dist` until code 256.
pub(super) fn inflate_block(
    r: &mut BitReader<'_>,
    out: &mut Vec<u8>,
    lit: &Huffman,
    dist: &Huffman,
) -> Result<(), InflateError> {
    loop {
        let sym = lit.decode(r)?;
        match sym {
            0..=255 => out.push(sym as u8),
            256 => return Ok(()),
            257..=285 => {
                let i = (sym - 257) as usize;
                let length = LEN_BASE[i] as usize + r.bits(LEN_EXTRA[i] as u32)? as usize;
                let dsym = dist.decode(r)? as usize;
                if dsym >= DIST_BASE.len() {
                    return Err(InflateError::Invalid);
                }
                let distance = DIST_BASE[dsym] as usize + r.bits(DIST_EXTRA[dsym] as u32)? as usize;
                if distance == 0 || distance > out.len() {
                    return Err(InflateError::Invalid);
                }
                // Copy byte by byte: source and destination overlap when the
                // distance is shorter than the length, which is how runs encode.
                let start = out.len() - distance;
                for k in 0..length {
                    let b = out[start + k];
                    out.push(b);
                }
            }
            _ => return Err(InflateError::Invalid),
        }
    }
}
