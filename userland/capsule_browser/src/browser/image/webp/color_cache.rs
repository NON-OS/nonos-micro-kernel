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

// VP8L color cache: a small hash-indexed set of recently emitted ARGB
// pixels, addressable by a short code so repeats cost few bits.
pub(super) struct ColorCache {
    entries: Vec<u32>,
    shift: u32,
}

impl ColorCache {
    pub fn new(bits: u32) -> Self {
        ColorCache { entries: vec![0u32; 1 << bits], shift: 32 - bits }
    }

    fn hash(&self, argb: u32) -> usize {
        (argb.wrapping_mul(0x1e35_a7bd) >> self.shift) as usize
    }

    pub fn insert(&mut self, argb: u32) {
        let h = self.hash(argb);
        self.entries[h] = argb;
    }

    pub fn lookup(&self, index: usize) -> u32 {
        self.entries.get(index).copied().unwrap_or(0)
    }
}
