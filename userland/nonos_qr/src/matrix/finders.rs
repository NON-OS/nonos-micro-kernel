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
    pub(super) fn place_finders(&mut self) {
        let n = self.n;
        for &(ox, oy) in &[(0usize, 0usize), (n - 7, 0), (0, n - 7)] {
            for dy in 0..7 {
                for dx in 0..7 {
                    let border = dx == 0 || dx == 6 || dy == 0 || dy == 6;
                    let core = (2..=4).contains(&dx) && (2..=4).contains(&dy);
                    self.set_fn(ox + dx, oy + dy, border || core);
                }
            }
        }
        // Separators: the light ring just outside each finder.
        for i in 0..8 {
            self.set_fn(7, i, false);
            self.set_fn(i, 7, false);
            self.set_fn(n - 8, i, false);
            self.set_fn(n - 1 - i, 7, false);
            self.set_fn(7, n - 8 + i, false);
            self.set_fn(i, n - 8, false);
        }
    }
}
