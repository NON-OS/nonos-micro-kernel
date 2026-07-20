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
use super::zeroing::zero_frame;

/// The top of the address space a 32-bit DMA descriptor can name (4GB). A DMA
/// allocation whose top crosses this cannot be handed to such a device.
const DMA_CEILING_32BIT: u64 = 0x1_0000_0000;

pub fn allocate_contiguous(
    state: &mut AllocatorState,
    frame_count: usize,
    flags: AllocFlags,
) -> Option<u64> {
    if frame_count == 0 || !state.is_initialized() {
        return None;
    }
    let total = state.frame_count;
    if frame_count > total {
        return None;
    }
    let (bptr, start) = (state.bitmap_ptr, state.frame_start);
    let run_start = if flags.contains(AllocFlags::HIGH) {
        unsafe { find_contiguous_high(bptr, total, frame_count)? }
    } else {
        unsafe { bitmap::find_contiguous_free(bptr, total, frame_count)? }
    };
    let offset = (run_start as u64).checked_mul(PAGE_SIZE_U64)?;
    let phys_addr = start.checked_add(offset)?;
    // A `DMA32` allocation must be addressable by a device that describes buffers
    // with a 32-bit physical address (Realtek/rtw88 TX buffer descriptors carry a
    // `__le32 dma`). If the run crosses 4GB the descriptor would silently truncate
    // it and the device would DMA the wrong memory, so refuse rather than hand back
    // an untruncatable frame. Checked before the bitmap is claimed so a refusal
    // leaves the allocator untouched. This applies only to `DMA32`, so 64-bit
    // devices and large high surfaces are unaffected.
    if flags.contains(AllocFlags::DMA32) {
        let span = (frame_count as u64).checked_mul(PAGE_SIZE_U64)?;
        if phys_addr.checked_add(span)? > DMA_CEILING_32BIT {
            return None;
        }
    }
    if !unsafe { bitmap::set_bit_range(bptr, run_start, frame_count) } {
        return None;
    }
    if flags.contains(AllocFlags::ZERO) {
        for j in 0..frame_count {
            zero_frame(Frame::new(phys_addr + (j as u64 * PAGE_SIZE_U64)));
        }
    }
    Some(phys_addr)
}

unsafe fn find_contiguous_high(ptr: *mut u8, total: usize, count: usize) -> Option<usize> {
    let mut run_length = 0usize;
    for i in (0..total).rev() {
        if unsafe { !bitmap::bit_test(ptr, i) } {
            run_length += 1;
            if run_length >= count {
                return Some(i);
            }
        } else {
            run_length = 0;
        }
    }
    None
}

pub fn free_contiguous(
    state: &mut AllocatorState,
    phys_addr: u64,
    frame_count: usize,
) -> PhysAllocResult<()> {
    if frame_count == 0 {
        return Ok(());
    }
    if !state.is_initialized() {
        return Err(PhysAllocError::NotInitialized);
    }
    let (start, total) = (state.frame_start, state.frame_count);
    if phys_addr < start {
        return Err(PhysAllocError::AddressBelowRange);
    }
    let offset = phys_addr.checked_sub(start).ok_or(PhysAllocError::AddressBelowRange)?;
    if offset % PAGE_SIZE_U64 != 0 {
        return Err(PhysAllocError::AddressNotAligned);
    }
    let start_idx = (offset / PAGE_SIZE_U64) as usize;
    let end_idx = start_idx.checked_add(frame_count).ok_or(PhysAllocError::RangeBeyondManaged)?;
    if end_idx > total {
        return Err(PhysAllocError::RangeBeyondManaged);
    }
    let bptr = state.bitmap_ptr;
    let allocated = unsafe { bitmap::is_range_allocated(bptr, start_idx, frame_count) };
    if allocated != Some(true) {
        return Err(PhysAllocError::DoubleFree);
    }
    if !unsafe { bitmap::clear_bit_range(bptr, start_idx, frame_count) } {
        return Err(PhysAllocError::RangeBeyondManaged);
    }
    Ok(())
}
