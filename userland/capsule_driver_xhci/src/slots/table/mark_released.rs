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
use super::types::SlotTable;
use super::valid::valid;

impl SlotTable {
    pub fn mark_released(&mut self, slot_id: u8, max_slots: u8) -> bool {
        if !valid(slot_id, max_slots) || !self.allocated[slot_id as usize] {
            return false;
        }
        self.addressed[slot_id as usize] = false;
        self.allocated[slot_id as usize] = false;
        self.count = self.count.saturating_sub(1);
        true
    }
}
