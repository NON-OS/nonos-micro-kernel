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
use super::SlotResources;
pub const XHCI_SLOT_TABLE_LEN: usize = 256;
pub struct SlotTable {
    allocated: [bool; XHCI_SLOT_TABLE_LEN],
    addressed: [bool; XHCI_SLOT_TABLE_LEN],
    resources: Vec<SlotResources>,
    count: u16,
}
impl SlotTable {
    pub fn new() -> Self {
        let empty = [false; XHCI_SLOT_TABLE_LEN];
        Self { allocated: empty, addressed: empty, resources: Vec::new(), count: 0 }
    }
    pub fn count(&self) -> u16 {
        self.count
    }
    pub fn mark_allocated(&mut self, slot_id: u8, max_slots: u8) -> bool {
        if !valid(slot_id, max_slots) || self.allocated[slot_id as usize] {
            return false;
        }
        self.allocated[slot_id as usize] = true;
        self.count = self.count.saturating_add(1);
        true
    }
    pub fn mark_released(&mut self, slot_id: u8, max_slots: u8) -> bool {
        if !valid(slot_id, max_slots) || !self.allocated[slot_id as usize] {
            return false;
        }
        self.addressed[slot_id as usize] = false;
        self.allocated[slot_id as usize] = false;
        self.count = self.count.saturating_sub(1);
        true
    }
    pub fn is_allocated(&self, slot_id: u8, max_slots: u8) -> bool {
        valid(slot_id, max_slots) && self.allocated[slot_id as usize]
    }
    pub fn is_addressed(&self, slot_id: u8, max_slots: u8) -> bool {
        valid(slot_id, max_slots) && self.addressed[slot_id as usize]
    }
    pub fn attach_addressed(&mut self, resources: SlotResources, max_slots: u8) -> bool {
        let slot_id = resources.slot_id;
        if !self.is_allocated(slot_id, max_slots) || self.is_addressed(slot_id, max_slots) {
            return false;
        }
        self.addressed[slot_id as usize] = true;
        self.resources.push(resources);
        true
    }
    pub fn take_resources(&mut self, slot_id: u8, max_slots: u8) -> Option<SlotResources> {
        if !valid(slot_id, max_slots) {
            return None;
        }
        self.addressed[slot_id as usize] = false;
        let idx = self.resources.iter().position(|r| r.slot_id == slot_id)?;
        Some(self.resources.swap_remove(idx))
    }
    pub fn resources_mut(&mut self, slot_id: u8, max_slots: u8) -> Option<&mut SlotResources> {
        if !self.is_addressed(slot_id, max_slots) {
            return None;
        }
        self.resources.iter_mut().find(|r| r.slot_id == slot_id)
    }
}
fn valid(slot_id: u8, max_slots: u8) -> bool {
    slot_id != 0 && slot_id <= max_slots
}