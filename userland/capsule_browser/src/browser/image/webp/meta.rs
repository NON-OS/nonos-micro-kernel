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

use alloc::vec::Vec;

// A meta-Huffman image splits the picture into blocks that each choose one of
// several Huffman groups. The block size is a power of two given by the
// huffman bits; a zero entropy image means a single group covers everything.
pub(super) struct Meta {
    pub image: Vec<u32>,
    pub bits: u32,
    pub xsize: usize,
}

// Cells across a dimension after right-shifting by the block bits.
pub(super) fn subsample_size(size: usize, bits: u32) -> usize {
    (size + (1 << bits) - 1) >> bits
}

impl Meta {
    // Highest group index referenced, so the caller knows how many groups to
    // read. The index lives in the low two bytes of each entropy pixel.
    pub fn group_count(&self) -> usize {
        self.image.iter().map(|p| ((p >> 8) & 0xffff) as usize).max().unwrap_or(0) + 1
    }

    // Group index for the block containing pixel (x, y).
    pub fn group_at(&self, x: usize, y: usize) -> usize {
        let hx = x >> self.bits;
        let hy = y >> self.bits;
        let p = self.image.get(hy * self.xsize + hx).copied().unwrap_or(0);
        ((p >> 8) & 0xffff) as usize
    }
}
