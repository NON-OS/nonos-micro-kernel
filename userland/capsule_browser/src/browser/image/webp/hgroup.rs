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
use super::read_huffman::read_huffman;

// The five Huffman trees a pixel is decoded with: green carries the literal
// green channel plus the length and color-cache codes, then one tree each for
// red, blue, alpha and the backward-reference distance.
pub(super) struct HGroup {
    pub green: Huffman,
    pub red: Huffman,
    pub blue: Huffman,
    pub alpha: Huffman,
    pub dist: Huffman,
}

// Distance alphabet is fixed at 40; the color channels are byte-valued.
const NUM_DISTANCE: usize = 40;
const NUM_LITERAL: usize = 256;
const NUM_LENGTH: usize = 24;

pub(super) fn read_hgroup(br: &mut BitReader, cache_bits: u32) -> Option<HGroup> {
    let cache = if cache_bits > 0 { 1usize << cache_bits } else { 0 };
    let green_alphabet = NUM_LITERAL + NUM_LENGTH + cache;
    Some(HGroup {
        green: read_huffman(br, green_alphabet)?,
        red: read_huffman(br, NUM_LITERAL)?,
        blue: read_huffman(br, NUM_LITERAL)?,
        alpha: read_huffman(br, NUM_LITERAL)?,
        dist: read_huffman(br, NUM_DISTANCE)?,
    })
}
