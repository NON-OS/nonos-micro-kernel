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

use super::super::constants::FRAME_SIZE;
use super::super::error::FrameResult;
use super::global::get_allocator;
use crate::memory::addr::PhysAddr;

pub fn allocate_frame() -> Option<PhysAddr> {
    let mut allocator = get_allocator().lock();
    if !allocator.is_initialized() {
        let _ = allocator.init();
    }
    // No DEFAULT_REGION seeding: `alloc()` draws only from the `phys` bitmap that
    // owns physical memory. Seeding a `[16 MiB, 512 MiB)` shadow region here is
    // what created the double-alloc/double-free aliasing.
    allocator.alloc()
}

pub fn deallocate_frame(addr: PhysAddr) -> FrameResult<()> {
    super::zero::zero_frame(addr);
    // Callers pass an address inside the frame, not always its base.
    get_allocator().lock().dealloc(addr.align_down(FRAME_SIZE))
}

pub fn add_memory_region(start: PhysAddr, end: PhysAddr) -> FrameResult<()> {
    get_allocator().lock().add_region(start, end)
}
