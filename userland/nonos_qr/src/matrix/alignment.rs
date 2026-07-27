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

use super::types::Matrix;
use crate::version::alignment_positions;

impl Matrix {
    pub(super) fn place_alignment(&mut self, version: u8) {
        let pos = alignment_positions(version);
        let n = self.n;
        for &cy in pos {
            for &cx in pos {
                let (cx, cy) = (cx as usize, cy as usize);
                let near_finder =
                    (cx < 8 && cy < 8) || (cx > n - 9 && cy < 8) || (cx < 8 && cy > n - 9);
                if near_finder {
                    continue;
                }
                for dy in -2i32..=2 {
                    for dx in -2i32..=2 {
                        let dark = dx.abs() == 2 || dy.abs() == 2 || (dx == 0 && dy == 0);
                        self.set_fn((cx as i32 + dx) as usize, (cy as i32 + dy) as usize, dark);
                    }
                }
            }
        }
    }
}
