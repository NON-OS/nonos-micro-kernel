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

extern crate alloc;

use alloc::vec::Vec;

use crate::hardware::broker::DeviceRecord;

use super::state::TABLE;

pub fn list() -> Vec<DeviceRecord> {
    TABLE.read().clone()
}

pub fn list_by_class(class: u32) -> Vec<DeviceRecord> {
    if class == 0 {
        return list();
    }
    TABLE.read().iter().filter(|r| r.class == class).copied().collect()
}

pub fn contains(device_id: u64) -> bool {
    TABLE.read().iter().any(|r| r.device_id == device_id)
}

pub fn class_of(device_id: u64) -> Option<u32> {
    TABLE.read().iter().find(|r| r.device_id == device_id).map(|r| r.class)
}
