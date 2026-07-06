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

use super::bitread::BitReader;
use super::huffman::Huffman;

// The order in which the 19 code-length-code lengths are transmitted, so the
// most common ones sit at the front of a short list.
const ORDER: [usize; 19] = [17, 18, 0, 1, 2, 3, 4, 5, 16, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

// Read the small Huffman tree that codes the per-symbol code lengths. A
// 4-bit count says how many 3-bit lengths follow, placed by the fixed order.
pub(super) fn read_code_len_code(br: &mut BitReader) -> Option<Huffman> {
    let count = br.read(4) as usize + 4;
    if count > ORDER.len() {
        return None;
    }
    let mut lengths = [0u8; 19];
    for &slot in ORDER.iter().take(count) {
        lengths[slot] = br.read(3) as u8;
    }
    if br.eos {
        return None;
    }
    Huffman::from_lengths(&lengths)
}
