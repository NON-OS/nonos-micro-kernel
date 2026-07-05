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

use crate::kernel_core::surface_registry::table::{bump_generation, SLOTS};
use crate::kernel_core::surface_registry::types::{encode_handle, SLOT_CAP};

pub fn release_owned_by_pid(pid: u32) -> u32 {
    let mut handles = [0u64; SLOT_CAP];
    let mut count = 0usize;
    {
        let mut slots = SLOTS.lock();
        for (idx, entry) in slots.iter_mut().enumerate() {
            let Some(slot) = entry.as_ref() else {
                continue;
            };
            if slot.owner_pid != pid {
                continue;
            }
            handles[count] = encode_handle(idx as u32, slot.epoch);
            count += 1;
            *entry = None;
            bump_generation(idx);
        }
    }
    for handle in handles.iter().take(count) {
        super::super::attach_map::forget_handle(*handle);
    }
    count as u32
}
