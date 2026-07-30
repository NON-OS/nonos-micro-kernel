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

use super::super::types::AllocatorStats;
use super::allocator::PageAllocator;
use spin::Mutex;

pub(super) static PAGE_ALLOCATOR: Mutex<PageAllocator> = Mutex::new(PageAllocator::new());
pub(super) static ALLOCATOR_STATS: AllocatorStats = AllocatorStats::new();

pub(super) fn get_timestamp() -> u64 {
    crate::arch::read_time_counter()
}
