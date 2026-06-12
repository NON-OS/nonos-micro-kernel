// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors

use core::mem::size_of;

use super::super::test_result::TestResult;
use crate::boot::handoff::api::validate_security;
use crate::boot::handoff::types::{
    flags, pixel_format, BootHandoffV1, FramebufferInfo, MemoryMapEntry,
};
use crate::memory::layout::constants::KERNEL_BASE;

pub(super) const VALID_ENTRY: u64 = KERNEL_BASE + 0x1000;

pub(super) fn baseline() -> BootHandoffV1 {
    let mut h = BootHandoffV1::default();
    h.rng.seed32 = [1u8; 32];
    h.entry_point = VALID_ENTRY;
    h
}

pub(super) fn baseline_with_fb() -> BootHandoffV1 {
    let mut h = baseline();
    h.flags |= flags::FB_AVAILABLE;
    h.fb = FramebufferInfo {
        ptr: 0x1000_0000,
        size: 1920 * 1080 * 4,
        width: 1920,
        height: 1080,
        stride: 1920 * 4,
        pixel_format: pixel_format::BGRX,
        cursor_y: 0,
        reserved: 0,
    };
    h
}

pub(super) fn baseline_with_mmap() -> BootHandoffV1 {
    let mut h = baseline();
    h.mmap.ptr = 0x2000_0000;
    h.mmap.entry_size = size_of::<MemoryMapEntry>() as u32;
    h.mmap.entry_count = 4;
    h
}

pub(crate) fn test_validate_security_accepts_baseline() -> TestResult {
    if validate_security(&baseline()).is_ok() {
        TestResult::Pass
    } else {
        TestResult::Fail
    }
}
