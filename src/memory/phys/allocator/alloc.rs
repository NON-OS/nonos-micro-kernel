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
use super::super::constants::PAGE_SIZE_U64;
use super::super::error::{PhysAllocError, PhysAllocResult};
use super::super::types::{AllocFlags, AllocatorState, Frame};
use super::random::mix64;
use super::zeroing::zero_frame;

pub fn allocate_frame(state: &mut AllocatorState, flags: AllocFlags) -> Option<Frame> {
    if !state.is_initialized() {
        return None;
    }
    let (bptr, start) = (state.bitmap_ptr, state.frame_start);
    let total = state.contig_split();
    if flags.contains(AllocFlags::HIGH) {
        for i in (0..total).rev() {
            if unsafe { !bitmap::bit_test(bptr, i) } {
                unsafe { bitmap::bit_set(bptr, i) };
                state.next_hint = i as u64;
                let frame = Frame::new(start.wrapping_add((i as u64).wrapping_mul(PAGE_SIZE_U64)));
                if flags.contains(AllocFlags::ZERO) {
                    zero_frame(frame);
                }
                return Some(frame);
            }
        }
        return None;
    }
    state.random_seed = state.random_seed.wrapping_add(1);
    let idx0 = (mix64(state.random_seed) as usize).wrapping_add(state.next_hint as usize) % total;
    for offset in 0..total {
        let i = (idx0 + offset) % total;
        if unsafe { !bitmap::bit_test(bptr, i) } {
            unsafe { bitmap::bit_set(bptr, i) };
            state.next_hint = i as u64;
            let frame = Frame::new(start.wrapping_add((i as u64).wrapping_mul(PAGE_SIZE_U64)));
            if flags.contains(AllocFlags::ZERO) {
                zero_frame(frame);
            }
            return Some(frame);
        }
    }
    None
}

pub fn deallocate_frame(state: &mut AllocatorState, frame: Frame) -> PhysAllocResult<()> {
    if !state.is_initialized() {
        return Err(PhysAllocError::NotInitialized);
    }
    let (start, total, bptr) = (state.frame_start, state.frame_count, state.bitmap_ptr);
    if frame.addr() < start {
        return Err(PhysAllocError::AddressBelowRange);
    }
    let offset = frame.addr().saturating_sub(start);
    if offset % PAGE_SIZE_U64 != 0 {
        return Err(PhysAllocError::AddressNotAligned);
    }
    let idx = (offset / PAGE_SIZE_U64) as usize;
    if idx >= total {
        return Err(PhysAllocError::AddressAboveRange);
    }
    if unsafe { !bitmap::bit_test(bptr, idx) } {
        return Err(PhysAllocError::DoubleFree);
    }
    unsafe { bitmap::bit_clear(bptr, idx) };
    Ok(())
}
