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

use super::super::constants::BOOTSTRAP_HEAP_SIZE;
use super::super::types::{BootstrapHeapMemory, HeapStatistics, SecureHeapAllocator};
use core::sync::atomic::AtomicBool;

#[global_allocator]
pub(crate) static KERNEL_HEAP: SecureHeapAllocator = SecureHeapAllocator::new();

pub static HEAP_ZERO_ON_ALLOC: AtomicBool = AtomicBool::new(true);
pub static HEAP_ZERO_ON_FREE: AtomicBool = AtomicBool::new(true);
pub static HEAP_STATS: HeapStatistics = HeapStatistics::new();
pub(super) static USING_BOOTSTRAP: AtomicBool = AtomicBool::new(true);

pub(super) static mut BOOTSTRAP_HEAP_MEMORY: BootstrapHeapMemory =
    BootstrapHeapMemory { data: [0u8; BOOTSTRAP_HEAP_SIZE] };

pub fn get_timestamp() -> u64 {
    crate::arch::read_time_counter()
}
