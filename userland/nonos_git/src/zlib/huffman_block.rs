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

//! A compressed block: decode symbols until the end-of-block code.

extern crate alloc;

use alloc::vec::Vec;

use super::bit_reader::BitReader;
use super::error::InflateError;
use super::huffman::Huffman;
use super::tables::{DIST_BASE, DIST_EXTRA, LEN_BASE, LEN_EXTRA};

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
            257..=285 => copy_back(r, out, sym, dist)?,
            _ => return Err(InflateError::Invalid),
        }
    }
}

/// A length and distance pair, copied from what has already been decoded.
fn copy_back(
    r: &mut BitReader<'_>,
    out: &mut Vec<u8>,
    sym: u16,
    dist: &Huffman,
) -> Result<(), InflateError> {
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
    // Byte at a time: source and destination overlap when the distance is
    // shorter than the length, which is how runs encode.
    let start = out.len() - distance;
    for k in 0..length {
        let b = out[start + k];
        out.push(b);
    }
    Ok(())
}
