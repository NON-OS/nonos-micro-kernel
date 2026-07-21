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

impl Matrix {
    // Mark the format-info modules around all three finders as reserved.
    pub(super) fn reserve_format(&mut self) {
        let n = self.n;
        for i in 0..9 {
            self.reserve(8, i);
            self.reserve(i, 8);
        }
        for i in 0..8 {
            self.reserve(n - 1 - i, 8);
            self.reserve(8, n - 1 - i);
        }
    }

    pub(super) fn reserve(&mut self, x: usize, y: usize) {
        self.function[y * self.n + x] = true;
    }

    // Reserve the two version-info blocks (versions >= 7).
    pub(super) fn reserve_version(&mut self) {
        let n = self.n;
        for i in 0..6 {
            for j in 0..3 {
                self.function[(n - 11 + j) * n + i] = true;
                self.function[i * n + (n - 11 + j)] = true;
            }
        }
    }
}
