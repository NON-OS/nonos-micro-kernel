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

use super::super::types::{HeapStats, SecureHeapAllocator};
use super::globals::{HEAP_STATS, HEAP_ZERO_ON_ALLOC, HEAP_ZERO_ON_FREE, KERNEL_HEAP};
use core::sync::atomic::Ordering;

pub fn set_heap_zero_on_alloc(enable: bool) {
    HEAP_ZERO_ON_ALLOC.store(enable, Ordering::SeqCst);
}
pub fn set_heap_zero_on_free(enable: bool) {
    HEAP_ZERO_ON_FREE.store(enable, Ordering::SeqCst);
}
pub fn get_heap_stats() -> HeapStats {
    HEAP_STATS.get_stats()
}

pub fn get_allocator() -> &'static SecureHeapAllocator {
    &KERNEL_HEAP
}
