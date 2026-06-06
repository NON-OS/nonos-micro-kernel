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

use super::layer::Layer;
use super::table::SceneTable;

impl SceneTable {
    pub fn drop_by_pid(&mut self, owner_pid: u32) -> u32 {
        let mut dropped = 0u32;
        for slot in self.entries.iter_mut() {
            if slot.in_use && slot.owner_pid == owner_pid {
                *slot = Layer::default();
                self.count = self.count.saturating_sub(1);
                dropped += 1;
            }
        }
        dropped
    }
}
