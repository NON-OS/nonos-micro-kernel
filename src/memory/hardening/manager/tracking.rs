// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use super::super::types::AllocationInfo;
use super::core::{MemoryHardening, HARDENING_STATS};
use core::sync::atomic::{AtomicU64, Ordering};

impl MemoryHardening {
    pub(super) fn track_allocation(&self, addr: u64, size: usize) -> u64 {
        let allocation_id = self.generate_allocation_id();
        let timestamp = crate::arch::read_time_counter();
        let info = AllocationInfo { size, timestamp, allocation_id, freed: false };
        self.allocation_tracker.lock().insert(addr, info);
        allocation_id
    }

    pub(super) fn track_deallocation(&self, addr: u64) -> Result<(), &'static str> {
        let mut tracker = self.allocation_tracker.lock();
        match tracker.get_mut(&addr) {
            Some(info) if info.freed => {
                HARDENING_STATS.increment_double_frees();
                Err("Double free detected")
            }
            Some(info) => {
                info.freed = true;
                Ok(())
            }
            None => {
                HARDENING_STATS.increment_use_after_free();
                Err("Use after free or invalid pointer")
            }
        }
    }

    pub(super) fn generate_allocation_id(&self) -> u64 {
        static ALLOC_COUNTER: AtomicU64 = AtomicU64::new(1);
        ALLOC_COUNTER.fetch_add(1, Ordering::Relaxed)
    }
}
