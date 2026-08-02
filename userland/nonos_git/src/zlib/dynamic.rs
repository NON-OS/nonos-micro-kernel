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

/// Build the literal/length and distance decoders from a dynamic header.
pub(super) fn dynamic_tables(r: &mut BitReader<'_>) -> Result<(Huffman, Huffman), InflateError> {
    let hlit = r.bits(5)? as usize + 257;
    let hdist = r.bits(5)? as usize + 1;
    let hclen = r.bits(4)? as usize + 4;
    if hlit > 286 || hdist > 30 {
        return Err(InflateError::Invalid);
    }

    // The code-length code lengths arrive in the permuted order.
    let mut cl_lengths = [0u8; 19];
    for &pos in CODE_LENGTH_ORDER.iter().take(hclen) {
        cl_lengths[pos] = r.bits(3)? as u8;
    }
    let cl = Huffman::new(&cl_lengths);

    // Decode the literal and distance code lengths as one run, honoring the
    // repeat codes 16 (copy previous), 17 and 18 (runs of zero).
    let mut lengths: Vec<u8> = Vec::with_capacity(hlit + hdist);
    while lengths.len() < hlit + hdist {
        let sym = cl.decode(r)?;
        match sym {
            0..=15 => lengths.push(sym as u8),
            16 => {
                let prev = *lengths.last().ok_or(InflateError::Invalid)?;
                let repeat = 3 + r.bits(2)? as usize;
                for _ in 0..repeat {
                    lengths.push(prev);
                }
            }
            17 => {
                let repeat = 3 + r.bits(3)? as usize;
                lengths.resize(lengths.len() + repeat, 0);
            }
            18 => {
                let repeat = 11 + r.bits(7)? as usize;
                lengths.resize(lengths.len() + repeat, 0);
            }
            _ => return Err(InflateError::Invalid),
        }
    }
    if lengths.len() != hlit + hdist {
        return Err(InflateError::Invalid);
    }

    let lit = Huffman::new(&lengths[..hlit]);
    let dist = Huffman::new(&lengths[hlit..]);
    Ok((lit, dist))
}
