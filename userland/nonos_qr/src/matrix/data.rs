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

use super::types::Matrix;

impl Matrix {
    /// Lay the interleaved codeword bits into the non-function modules in the
    /// standard right-to-left, upward-then-downward serpentine.
    pub(crate) fn place_data(&mut self, codewords: &[u8]) {
        let n = self.n;
        let mut bit = 0usize;
        let total = codewords.len() * 8;
        let mut col = n as i32 - 1;
        let mut upward = true;
        while col > 0 {
            if col == 6 {
                col -= 1;
            }
            let rows: Vec<i32> =
                if upward { (0..n as i32).rev().collect() } else { (0..n as i32).collect() };
            for y in rows {
                for dx in 0..2 {
                    let x = col - dx;
                    let (xi, yi) = (x as usize, y as usize);
                    if self.function[yi * n + xi] {
                        continue;
                    }
                    let dark = if bit < total {
                        let b = codewords[bit / 8];
                        (b >> (7 - (bit % 8))) & 1 == 1
                    } else {
                        false
                    };
                    self.modules[yi * n + xi] = dark;
                    bit += 1;
                }
            }
            upward = !upward;
            col -= 2;
        }
    }
}
