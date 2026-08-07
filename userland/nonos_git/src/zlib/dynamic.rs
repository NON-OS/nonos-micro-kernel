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

//! Reading a dynamic block's header and building its two decoders.

extern crate alloc;

use alloc::vec::Vec;

use super::bit_reader::BitReader;
use super::error::InflateError;
use super::huffman::Huffman;
use super::tables::CODE_LENGTH_ORDER;

pub(super) fn dynamic_tables(r: &mut BitReader<'_>) -> Result<(Huffman, Huffman), InflateError> {
    let hlit = r.bits(5)? as usize + 257;
    let hdist = r.bits(5)? as usize + 1;
    let hclen = r.bits(4)? as usize + 4;
    if hlit > 286 || hdist > 30 {
        return Err(InflateError::Invalid);
    }

    let mut cl_lengths = [0u8; 19];
    for &pos in CODE_LENGTH_ORDER.iter().take(hclen) {
        cl_lengths[pos] = r.bits(3)? as u8;
    }
    let lengths = code_lengths(r, &Huffman::new(&cl_lengths), hlit + hdist)?;

    Ok((Huffman::new(&lengths[..hlit]), Huffman::new(&lengths[hlit..])))
}

/// One run of lengths, honoring repeat codes 16, 17 and 18.
fn code_lengths(
    r: &mut BitReader<'_>,
    cl: &Huffman,
    total: usize,
) -> Result<Vec<u8>, InflateError> {
    let mut lengths: Vec<u8> = Vec::with_capacity(total);
    while lengths.len() < total {
        match cl.decode(r)? {
            s @ 0..=15 => lengths.push(s as u8),
            16 => {
                let prev = *lengths.last().ok_or(InflateError::Invalid)?;
                let n = 3 + r.bits(2)? as usize;
                lengths.resize(lengths.len() + n, prev);
            }
            17 => {
                let n = 3 + r.bits(3)? as usize;
                lengths.resize(lengths.len() + n, 0);
            }
            18 => {
                let n = 11 + r.bits(7)? as usize;
                lengths.resize(lengths.len() + n, 0);
            }
            _ => return Err(InflateError::Invalid),
        }
    }
    if lengths.len() != total {
        return Err(InflateError::Invalid);
    }
    Ok(lengths)
}
