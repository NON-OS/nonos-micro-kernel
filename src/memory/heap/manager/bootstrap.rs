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
use super::globals::{BOOTSTRAP_HEAP_MEMORY, HEAP_STATS, KERNEL_HEAP, USING_BOOTSTRAP};
use core::ptr::addr_of_mut;
use core::sync::atomic::Ordering;

pub fn init_bootstrap() {
    if !KERNEL_HEAP.is_initialized() {
        let heap_start = unsafe {
            let ptr = addr_of_mut!(BOOTSTRAP_HEAP_MEMORY);
            (*ptr).data.as_mut_ptr()
        };
        unsafe {
            KERNEL_HEAP.init(heap_start, BOOTSTRAP_HEAP_SIZE);
        }
        HEAP_STATS.set_total_size(BOOTSTRAP_HEAP_SIZE);
        USING_BOOTSTRAP.store(true, Ordering::Release);
    }
}

#[inline]
pub fn is_using_bootstrap() -> bool {
    USING_BOOTSTRAP.load(Ordering::Acquire)
}
