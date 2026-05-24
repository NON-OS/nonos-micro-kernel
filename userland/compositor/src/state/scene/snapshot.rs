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

use super::layer::{Layer, MAX_LAYERS};
use super::table::SceneTable;

impl SceneTable {
    pub fn z_sorted_snapshot(&self) -> ([Layer; MAX_LAYERS], usize) {
        let mut out = [Layer::default(); MAX_LAYERS];
        let mut n = 0;
        for layer in self.entries.iter().filter(|l| l.in_use) {
            out[n] = *layer;
            n += 1;
        }
        let mut i = 1;
        while i < n {
            let mut j = i;
            while j > 0 && out[j - 1].z > out[j].z {
                out.swap(j - 1, j);
                j -= 1;
            }
            i += 1;
        }
        (out, n)
    }
}
