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

extern crate alloc;

pub mod allocator;
mod bitmap;
pub mod constants;
pub mod error;
mod types;

#[cfg(test)]
mod tests;

pub use allocator::{
    phys_alloc as alloc, phys_alloc_contiguous as alloc_contiguous,
    phys_allocate_frame as allocate_frame, phys_deallocate_frame as deallocate_frame,
    phys_find_first_free as find_first_free, phys_free as free,
    phys_free_contiguous as free_contiguous, phys_init as init,
    phys_init_with_bitmap as init_with_bitmap, phys_is_initialized as is_initialized,
    phys_largest_free_run as largest_free_run, phys_managed_range as managed_range,
    phys_total_free_frames as total_free_frames, phys_total_memory as total_memory,
    phys_zone_stats as zone_stats,
};

pub fn free_memory() -> u64 {
    total_memory().saturating_sub((total_free_frames() as u64) * 4096)
}
pub use constants::*;
pub use error::{PhysAllocError, PhysAllocResult};
pub use types::{AllocFlags, AllocatorState, Frame, PhysFrame, ZoneStats};
