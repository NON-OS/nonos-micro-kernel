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

//! Physical Memory Allocator Unit Tests

use super::*;

// ============================================================================
// CONSTANT TESTS
// ============================================================================

#[test]
fn test_page_size() {
    assert_eq!(PAGE_SIZE, 4096);
    assert_eq!(PAGE_SIZE_U64, 4096u64);
}

#[test]
fn test_bitmap_constants() {
    assert_eq!(BITS_PER_BYTE, 8);
    assert!(MAX_PHYSICAL_MEMORY > 0);
    assert!(MAX_FRAME_COUNT > 0);
    assert!(MAX_BITMAP_SIZE > 0);
}

#[test]
fn test_alignment_helpers() {
    assert_eq!(align_up(0, 4096), 0);
    assert_eq!(align_up(1, 4096), 4096);
    assert_eq!(align_up(4096, 4096), 4096);
    assert_eq!(align_up(4097, 4096), 8192);

    assert_eq!(align_down(0, 4096), 0);
    assert_eq!(align_down(1, 4096), 0);
    assert_eq!(align_down(4096, 4096), 4096);
    assert_eq!(align_down(4097, 4096), 4096);
}

#[test]
fn test_bitmap_bytes_calculation() {
    assert_eq!(bitmap_bytes_for_frames(0), 0);
    assert_eq!(bitmap_bytes_for_frames(1), 1);
    assert_eq!(bitmap_bytes_for_frames(8), 1);
    assert_eq!(bitmap_bytes_for_frames(9), 2);
    assert_eq!(bitmap_bytes_for_frames(16), 2);
    assert_eq!(bitmap_bytes_for_frames(17), 3);
}

#[test]
fn test_frames_in_range() {
    assert_eq!(frames_in_range(0, 0), 0);
    assert_eq!(frames_in_range(4096, 0), 0);
    assert_eq!(frames_in_range(0, 4096), 1);
    assert_eq!(frames_in_range(0, 8192), 2);
    assert_eq!(frames_in_range(0, 16 * 4096), 16);
}

// ============================================================================
// TYPE TESTS
// ============================================================================

#[test]
fn test_alloc_flags() {
    assert_eq!(AllocFlags::EMPTY.bits(), 0);
    assert_eq!(AllocFlags::ZERO.bits(), 1);
    assert_eq!(AllocFlags::HIGH.bits(), 2);
    assert_eq!(AllocFlags::DMA.bits(), 4);
    assert_eq!(AllocFlags::CONTIGUOUS.bits(), 8);
}

#[test]
fn test_alloc_flags_combinations() {
    let flags = AllocFlags::ZERO | AllocFlags::HIGH;
    assert!(flags.contains(AllocFlags::ZERO));
    assert!(flags.contains(AllocFlags::HIGH));
    assert!(!flags.contains(AllocFlags::DMA));
}

#[test]
fn test_frame_creation() {
    let frame = Frame::new(0x1000);
    assert_eq!(frame.addr(), 0x1000);
    assert!(!frame.is_null());

    let null_frame = Frame::new(0);
    assert!(null_frame.is_null());
}

#[test]
fn test_frame_conversions() {
    let frame = Frame::new(0x2000);

    let addr: u64 = frame.into();
    assert_eq!(addr, 0x2000);

    let frame2: Frame = 0x3000u64.into();
    assert_eq!(frame2.addr(), 0x3000);
}

#[test]
fn test_frame_number() {
    let frame = Frame::new(0x10000);
    assert_eq!(frame.number(0, 4096), 16);
    assert_eq!(frame.number(0x8000, 4096), 8);
}

#[test]
fn test_zone_stats() {
    let stats = ZoneStats::new(100, 60);
    assert_eq!(stats.frames_total, 100);
    assert_eq!(stats.frames_free, 60);
    assert_eq!(stats.frames_allocated(), 40);
    assert_eq!(stats.usage_percent(), 40);
    assert_eq!(stats.total_bytes(4096), 100 * 4096);
    assert_eq!(stats.free_bytes(4096), 60 * 4096);
}

#[test]
fn test_zone_stats_empty() {
    let stats = ZoneStats::new(0, 0);
    assert_eq!(stats.usage_percent(), 0);
}

#[test]
fn test_allocator_state_new() {
    let state = AllocatorState::new();
    assert!(!state.is_initialized());
    assert_eq!(state.frame_start, 0);
    assert_eq!(state.frame_count, 0);
}

// ============================================================================
// ERROR TESTS
// ============================================================================

#[test]
fn test_error_as_str() {
    assert_eq!(
        PhysAllocError::NotInitialized.as_str(),
        "Physical memory allocator not initialized"
    );
    assert_eq!(PhysAllocError::DoubleFree.as_str(), "Double free detected");
}

#[test]
fn test_error_is_fatal() {
    assert!(PhysAllocError::OutOfMemory.is_fatal());
    assert!(PhysAllocError::NotInitialized.is_fatal());
    assert!(!PhysAllocError::DoubleFree.is_fatal());
}

#[test]
fn test_error_indicates_corruption() {
    assert!(PhysAllocError::DoubleFree.indicates_corruption());
    assert!(PhysAllocError::FrameNotAllocated.indicates_corruption());
    assert!(!PhysAllocError::InvalidRange.indicates_corruption());
}

#[test]
fn test_error_display() {
    let error = PhysAllocError::InvalidRange;
    let msg = format!("{}", error);
    assert_eq!(msg, "Physical memory range invalid: end <= start");
}

// ============================================================================
// BITMAP TESTS
// ============================================================================

#[test]
fn test_bitmap_bit_operations() {
    let mut bitmap = [0u8; 2];
    let ptr = bitmap.as_mut_ptr();

    // SAFETY: bitmap is valid and indices are in bounds
    unsafe {
        assert!(!bitmap::bit_test(ptr, 0));
        assert!(!bitmap::bit_test(ptr, 7));
        assert!(!bitmap::bit_test(ptr, 8));

        bitmap::bit_set(ptr, 0);
        assert!(bitmap::bit_test(ptr, 0));

        bitmap::bit_set(ptr, 7);
        assert!(bitmap::bit_test(ptr, 7));

        bitmap::bit_set(ptr, 8);
        assert!(bitmap::bit_test(ptr, 8));

        bitmap::bit_clear(ptr, 0);
        assert!(!bitmap::bit_test(ptr, 0));
        assert!(bitmap::bit_test(ptr, 7));
    }
}

#[test]
fn test_bitmap_count_free() {
    let mut bitmap = [0u8; 2];
    let ptr = bitmap.as_mut_ptr();

    // SAFETY: bitmap is valid
    unsafe {
        assert_eq!(bitmap::count_free_bits(ptr, 16), 16);

        bitmap::bit_set(ptr, 0);
        bitmap::bit_set(ptr, 5);
        bitmap::bit_set(ptr, 10);
        assert_eq!(bitmap::count_free_bits(ptr, 16), 13);
    }
}

#[test]
fn test_bitmap_find_contiguous() {
    let mut bitmap = [0u8; 4];
    let ptr = bitmap.as_mut_ptr();

    // SAFETY: bitmap is valid
    unsafe {
        // All free, should find at start
        let result = bitmap::find_contiguous_free(ptr, 32, 4);
        assert_eq!(result, Some(0));

        // Set some bits, then search
        bitmap::bit_set(ptr, 0);
        bitmap::bit_set(ptr, 1);
        let result = bitmap::find_contiguous_free(ptr, 32, 4);
        assert_eq!(result, Some(2));

        // Fragment more
        bitmap::bit_set(ptr, 4);
        let result = bitmap::find_contiguous_free(ptr, 32, 4);
        assert_eq!(result, Some(5));
    }
}

#[test]
fn test_bitmap_set_range() {
    let mut bitmap = [0u8; 2];
    let ptr = bitmap.as_mut_ptr();

    // SAFETY: bitmap is valid
    unsafe {
        bitmap::set_bit_range(ptr, 2, 5);
        assert!(!bitmap::bit_test(ptr, 0));
        assert!(!bitmap::bit_test(ptr, 1));
        assert!(bitmap::bit_test(ptr, 2));
        assert!(bitmap::bit_test(ptr, 3));
        assert!(bitmap::bit_test(ptr, 4));
        assert!(bitmap::bit_test(ptr, 5));
        assert!(bitmap::bit_test(ptr, 6));
        assert!(!bitmap::bit_test(ptr, 7));
    }
}

// ============================================================================
// ALLOCATOR TESTS
// ============================================================================

#[test]
fn test_allocator_init() {
    use crate::memory::addr::PhysAddr;

    let mut state = AllocatorState::new();
    let mut bitmap = [0u8; 2];

    let start = PhysAddr::new(0x1000_0000);
    let end = PhysAddr::new(0x1000_0000 + 16 * 4096);

    let result =
        allocator::init_with_bitmap(&mut state, start, end, bitmap.as_mut_ptr(), bitmap.len());

    assert!(result.is_ok());
    assert!(state.is_initialized());
    assert_eq!(state.frame_count, 16);
}

#[test]
fn test_allocator_init_invalid_range() {
    use crate::memory::addr::PhysAddr;

    let mut state = AllocatorState::new();
    let mut bitmap = [0u8; 2];

    let start = PhysAddr::new(0x2000_0000);
    let end = PhysAddr::new(0x1000_0000); // end < start

    let result =
        allocator::init_with_bitmap(&mut state, start, end, bitmap.as_mut_ptr(), bitmap.len());

    assert_eq!(result, Err(PhysAllocError::InvalidRange));
}

#[test]
fn test_allocator_basic_alloc_free() {
    use crate::memory::addr::PhysAddr;

    let mut state = AllocatorState::new();
    let mut bitmap = [0u8; 2];

    let start = PhysAddr::new(0x1000_0000);
    let end = PhysAddr::new(0x1000_0000 + 16 * 4096);

    allocator::init_with_bitmap(&mut state, start, end, bitmap.as_mut_ptr(), bitmap.len())
        .expect("init");

    let f1 = allocator::allocate_frame(&mut state, AllocFlags::EMPTY).expect("alloc1");
    let f2 = allocator::allocate_frame(&mut state, AllocFlags::EMPTY).expect("alloc2");

    assert_ne!(f1.addr(), f2.addr());

    allocator::deallocate_frame(&mut state, f1).expect("free1");
    allocator::deallocate_frame(&mut state, f2).expect("free2");
}

#[test]
fn test_allocator_double_free() {
    use crate::memory::addr::PhysAddr;

    let mut state = AllocatorState::new();
    let mut bitmap = [0u8; 1];

    let start = PhysAddr::new(0x2000_0000);
    let end = PhysAddr::new(0x2000_0000 + 8 * 4096);

    allocator::init_with_bitmap(&mut state, start, end, bitmap.as_mut_ptr(), bitmap.len())
        .expect("init");

    let frame = allocator::allocate_frame(&mut state, AllocFlags::EMPTY).expect("alloc");
    allocator::deallocate_frame(&mut state, frame).expect("free");

    let result = allocator::deallocate_frame(&mut state, frame);
    assert_eq!(result, Err(PhysAllocError::DoubleFree));
}

#[test]
fn test_allocator_contiguous() {
    use crate::memory::addr::PhysAddr;

    let mut state = AllocatorState::new();
    let mut bitmap = [0u8; 4];

    let start = PhysAddr::new(0x3000_0000);
    let end = PhysAddr::new(0x3000_0000 + 32 * 4096);

    allocator::init_with_bitmap(&mut state, start, end, bitmap.as_mut_ptr(), bitmap.len())
        .expect("init");

    let addr = allocator::allocate_contiguous(&mut state, 4, AllocFlags::EMPTY).expect("alloc");
    assert_eq!(addr, 0x3000_0000);

    allocator::free_contiguous(&mut state, addr, 4).expect("free");
}

#[test]
fn test_allocator_contiguous_high() {
    use crate::memory::addr::PhysAddr;

    let mut state = AllocatorState::new();
    let mut bitmap = [0u8; 4];

    let start = PhysAddr::new(0x3100_0000);
    let end = PhysAddr::new(0x3100_0000 + 32 * 4096);

    allocator::init_with_bitmap(&mut state, start, end, bitmap.as_mut_ptr(), bitmap.len())
        .expect("init");

    let addr = allocator::allocate_contiguous(&mut state, 4, AllocFlags::HIGH).expect("alloc");
    assert_eq!(addr, 0x3100_0000 + 28 * 4096);

    allocator::free_contiguous(&mut state, addr, 4).expect("free");
}

#[test]
fn test_allocator_exhaust() {
    use crate::memory::addr::PhysAddr;

    let mut state = AllocatorState::new();
    let mut bitmap = [0u8; 1];

    let start = PhysAddr::new(0x4000_0000);
    let end = PhysAddr::new(0x4000_0000 + 8 * 4096);

    allocator::init_with_bitmap(&mut state, start, end, bitmap.as_mut_ptr(), bitmap.len())
        .expect("init");

    // Allocate all 8 frames
    for _ in 0..8 {
        assert!(allocator::allocate_frame(&mut state, AllocFlags::EMPTY).is_some());
    }

    // 9th allocation should fail
    assert!(allocator::allocate_frame(&mut state, AllocFlags::EMPTY).is_none());
}

#[test]
fn test_allocator_zone_stats() {
    use crate::memory::addr::PhysAddr;

    let mut state = AllocatorState::new();
    let mut bitmap = [0u8; 2];

    let start = PhysAddr::new(0x5000_0000);
    let end = PhysAddr::new(0x5000_0000 + 16 * 4096);

    allocator::init_with_bitmap(&mut state, start, end, bitmap.as_mut_ptr(), bitmap.len())
        .expect("init");

    let stats = allocator::get_zone_stats(&state);
    assert_eq!(stats.frames_total, 16);
    assert_eq!(stats.frames_free, 16);

    let _f1 = allocator::allocate_frame(&mut state, AllocFlags::EMPTY);
    let _f2 = allocator::allocate_frame(&mut state, AllocFlags::EMPTY);

    let stats = allocator::get_zone_stats(&state);
    assert_eq!(stats.frames_total, 16);
    assert_eq!(stats.frames_free, 14);
}

// ============================================================================
// RANDOMIZATION TESTS
// ============================================================================

#[test]
fn test_mix64() {
    // Mix should produce different outputs for different inputs
    let a = allocator::mix64(1);
    let b = allocator::mix64(2);
    assert_ne!(a, b);

    // Same input should give same output
    let c = allocator::mix64(1);
    assert_eq!(a, c);
}
