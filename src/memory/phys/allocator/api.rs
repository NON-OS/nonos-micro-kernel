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
use super::super::bitmap;
use super::super::constants::{bitmap_bytes_for_frames, BITS_PER_BYTE, PAGE_SIZE};
use super::super::error::PhysAllocResult;
use super::super::types::{AllocFlags, AllocatorState, Frame, ZoneStats};
use super::{
    allocate_contiguous, allocate_frame, deallocate_frame, free_contiguous, get_zone_stats,
    init_with_bitmap, largest_free_run, managed_range, total_memory,
};
use crate::memory::addr::PhysAddr;
use spin::Mutex;

static ALLOCATOR: Mutex<AllocatorState> = Mutex::new(AllocatorState::new());

pub fn phys_init_with_bitmap(
    managed_start: PhysAddr,
    managed_end: PhysAddr,
    bitmap_ptr: *mut u8,
    bitmap_bytes: usize,
) -> PhysAllocResult<()> {
    init_with_bitmap(&mut ALLOCATOR.lock(), managed_start, managed_end, bitmap_ptr, bitmap_bytes)
}

pub fn phys_init(managed_start: PhysAddr, managed_end: PhysAddr) -> PhysAllocResult<()> {
    let size = ((managed_end.as_u64().saturating_sub(managed_start.as_u64())) as usize / PAGE_SIZE)
        + BITS_PER_BYTE;
    let bytes = bitmap_bytes_for_frames(size);
    let mut v = alloc::vec::Vec::new();
    v.resize(bytes, 0u8);
    phys_init_with_bitmap(managed_start, managed_end, v.leak().as_mut_ptr(), bytes)
}

pub fn phys_allocate_frame(flags: AllocFlags) -> Option<Frame> {
    allocate_frame(&mut ALLOCATOR.lock(), flags)
}
pub fn phys_deallocate_frame(frame: Frame) -> PhysAllocResult<()> {
    deallocate_frame(&mut ALLOCATOR.lock(), frame)
}
pub fn phys_alloc_contiguous(frame_count: usize, flags: AllocFlags) -> Option<u64> {
    allocate_contiguous(&mut ALLOCATOR.lock(), frame_count, flags)
}
pub fn phys_free_contiguous(phys_addr: u64, frame_count: usize) -> PhysAllocResult<()> {
    free_contiguous(&mut ALLOCATOR.lock(), phys_addr, frame_count)
}
#[inline]
pub fn phys_alloc(flags: AllocFlags) -> Option<Frame> {
    phys_allocate_frame(flags)
}
#[inline]
pub fn phys_free(frame: Frame) -> PhysAllocResult<()> {
    phys_deallocate_frame(frame)
}
pub fn phys_zone_stats() -> alloc::vec::Vec<(u32, ZoneStats)> {
    alloc::vec![(0, get_zone_stats(&ALLOCATOR.lock()))]
}
pub fn phys_total_free_frames() -> usize {
    get_zone_stats(&ALLOCATOR.lock()).frames_free
}
pub fn phys_total_memory() -> u64 {
    total_memory(&ALLOCATOR.lock())
}
pub fn phys_largest_free_run() -> usize {
    largest_free_run(&ALLOCATOR.lock())
}
pub fn phys_managed_range() -> (u64, u64) {
    managed_range(&ALLOCATOR.lock())
}
pub fn phys_is_initialized() -> bool {
    ALLOCATOR.lock().is_initialized()
}

pub fn phys_find_first_free(start_from: usize) -> Option<usize> {
    let state = ALLOCATOR.lock();
    if !state.is_initialized() {
        return None;
    }
    unsafe { bitmap::find_first_free(state.bitmap_ptr, state.frame_count, start_from) }
}
