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

use super::bitread::BitReader;
use super::code_len_code::read_code_len_code;
use super::huffman::Huffman;
use super::read_code_lengths::read_code_lengths;

// One Huffman tree from the VP8L bitstream. The simple form spells out one or
// two symbols directly; the normal form transmits per-symbol code lengths,
// themselves Huffman-coded by a small code-length alphabet.
pub(super) fn read_huffman(br: &mut BitReader, alphabet: usize) -> Option<Huffman> {
    if br.read_bit() == 1 {
        return read_simple(br, alphabet);
    }
    let cl_code = read_code_len_code(br)?;
    let lengths = read_code_lengths(br, &cl_code, alphabet)?;
    Huffman::from_lengths(&lengths)
}

// Simple code: one or two symbols, each given length 1 so a single bit (or
// none) selects between them.
fn read_simple(br: &mut BitReader, alphabet: usize) -> Option<Huffman> {
    let two = br.read_bit() == 1;
    let first_wide = br.read_bit() == 1;
    let s0 = br.read(if first_wide { 8 } else { 1 }) as usize;
    let mut lengths = vec![0u8; alphabet];
    *lengths.get_mut(s0)? = 1;
    if two {
        let s1 = br.read(8) as usize;
        *lengths.get_mut(s1)? = 1;
    }
    Huffman::from_lengths(&lengths)
}
