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

use alloc::vec;
use alloc::vec::Vec;

use super::bitread::BitReader;

// A canonical Huffman decoder over symbol code lengths, walked bit by bit.
// VP8L uses one of these per stream (green+length, red, blue, alpha, dist).
pub(super) struct Huffman {
    // For each code length L, the first canonical code and the symbol offset.
    counts: [u32; MAX_LEN + 1],
    symbols: Vec<u16>,
}

const MAX_LEN: usize = 15;

impl Huffman {
    // Build from per-symbol code lengths (0 means the symbol is absent).
    pub fn from_lengths(lengths: &[u8]) -> Option<Self> {
        let mut counts = [0u32; MAX_LEN + 1];
        for &l in lengths {
            if l as usize > MAX_LEN {
                return None;
            }
            counts[l as usize] += 1;
        }
        counts[0] = 0;
        // Sort symbols by (length, symbol) into canonical order.
        let mut offsets = [0u32; MAX_LEN + 2];
        for l in 1..=MAX_LEN {
            offsets[l + 1] = offsets[l] + counts[l];
        }
        let total = offsets[MAX_LEN + 1] as usize;
        if total == 0 {
            return None;
        }
        let mut symbols = vec![0u16; total];
        let mut next = offsets;
        for (sym, &l) in lengths.iter().enumerate() {
            if l != 0 {
                symbols[next[l as usize] as usize] = sym as u16;
                next[l as usize] += 1;
            }
        }
        Some(Huffman { counts, symbols })
    }

    // A single-symbol tree encodes a constant; the reader consumes no bits.
    pub fn single(&self) -> Option<u16> {
        (self.symbols.len() == 1).then(|| self.symbols[0])
    }

    // Decode one symbol, descending the canonical table by length.
    pub fn read(&self, br: &mut BitReader) -> u16 {
        if let Some(s) = self.single() {
            return s;
        }
        let mut code = 0i32;
        let mut first = 0i32;
        let mut index = 0i32;
        for len in 1..=MAX_LEN {
            code |= br.read_bit() as i32;
            let count = self.counts[len] as i32;
            if code - first < count {
                // Bounds-checked: an oversubscribed (invalid) code-length table
                // from a crafted WEBP can drive this index past symbols.len().
                // Return 0 (treated as failure/eos by callers) instead of
                // panicking, matching the safe HTTP-inflate decoder.
                return self.symbols.get((index + (code - first)) as usize).copied().unwrap_or(0);
            }
            index += count;
            first += count;
            first <<= 1;
            code <<= 1;
        }
        0
    }
}
