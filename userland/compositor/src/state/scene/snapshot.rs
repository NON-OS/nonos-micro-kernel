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
    // Bottom-to-top draw order. Layers rank by z first; within the same z the
    // window owned by `focused_pid` sorts last, so it draws on top. This is what
    // makes a click raise a window: the click focuses it, and focus lifts it
    // above its peers without crossing into a higher band such as the taskbar.
    // A stable insertion sort keeps every other window in its existing order.
    pub fn z_sorted_snapshot(&self, focused_pid: u32) -> ([Layer; MAX_LAYERS], usize) {
        let mut out = [Layer::default(); MAX_LAYERS];
        let mut n = 0;
        for layer in self.entries.iter().filter(|l| l.in_use) {
            out[n] = *layer;
            n += 1;
        }
        // Composite rank: z in the high bits, "is focused" in the low bit, so a
        // focused layer only outranks a peer at the same z, never one above it.
        let rank = |l: &Layer| -> u64 {
            ((l.z as u64) << 1) | (focused_pid != 0 && l.owner_pid == focused_pid) as u64
        };
        let mut i = 1;
        while i < n {
            let mut j = i;
            while j > 0 && rank(&out[j - 1]) > rank(&out[j]) {
                out.swap(j - 1, j);
                j -= 1;
            }
            i += 1;
        }
        (out, n)
    }
}
