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

use super::super::bitmap;
use super::super::constants::{PAGE_SIZE, PAGE_SIZE_U64};
use super::super::types::{AllocatorState, ZoneStats};

pub fn get_zone_stats(state: &AllocatorState) -> ZoneStats {
    if !state.is_initialized() {
        return ZoneStats::new(0, 0);
    }
    let free = unsafe { bitmap::count_free_bits(state.bitmap_ptr, state.frame_count) };
    ZoneStats::new(state.frame_count, free)
}

pub fn managed_range(state: &AllocatorState) -> (u64, u64) {
    let start = state.frame_start;
    let size = (state.frame_count as u64).saturating_mul(PAGE_SIZE_U64);
    let end = start.saturating_add(size);
    (start, end)
}

pub fn total_memory(state: &AllocatorState) -> u64 {
    (state.frame_count as u64).saturating_mul(PAGE_SIZE as u64)
}

pub fn largest_free_run(state: &AllocatorState) -> usize {
    if !state.is_initialized() {
        return 0;
    }
    let mut best = 0usize;
    let mut run = 0usize;
    for i in 0..state.frame_count {
        if unsafe { !bitmap::bit_test(state.bitmap_ptr, i) } {
            run += 1;
            if run > best {
                best = run;
            }
        } else {
            run = 0;
        }
    }
    best
}
