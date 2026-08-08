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

//! A canonical Huffman decoder built from a list of per-symbol code lengths.

extern crate alloc;

use alloc::vec::Vec;

use super::bit_reader::BitReader;
use super::error::InflateError;

pub(super) struct Huffman {
    /// counts[len] = number of codes of that bit length.
    counts: [u16; 16],
    /// symbols in canonical order.
    symbols: Vec<u16>,
}

impl Huffman {
    /// Build the decoder. Symbols with length zero are absent from the code.
    pub(super) fn new(lengths: &[u8]) -> Huffman {
        let mut counts = [0u16; 16];
        for &l in lengths {
            counts[l as usize] += 1;
        }
        counts[0] = 0;
        // Offset of the first symbol of each length in the sorted table.
        let mut offsets = [0u16; 16];
        for len in 1..16 {
            offsets[len] = offsets[len - 1] + counts[len - 1];
        }
        let mut symbols = alloc::vec![0u16; lengths.len()];
        for (sym, &len) in lengths.iter().enumerate() {
            if len != 0 {
                symbols[offsets[len as usize] as usize] = sym as u16;
                offsets[len as usize] += 1;
            }
        }
        Huffman { counts, symbols }
    }

    /// Decode one symbol, reading one bit per level as the canonical code
    /// grows, so an over-long or absent code is rejected rather than looping.
    pub(super) fn decode(&self, r: &mut BitReader<'_>) -> Result<u16, InflateError> {
        let mut code: i32 = 0;
        let mut first: i32 = 0;
        let mut index: i32 = 0;
        for len in 1..16 {
            code |= r.bit()? as i32;
            let count = self.counts[len] as i32;
            if code - first < count {
                return Ok(self.symbols[(index + (code - first)) as usize]);
            }
            index += count;
            first += count;
            first <<= 1;
            code <<= 1;
        }
        Err(InflateError::Invalid)
    }
}
