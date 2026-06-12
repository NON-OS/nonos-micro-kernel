// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors

use core::mem::size_of;

use super::super::test_result::TestResult;
use crate::boot::handoff::api::{validate_security, HandoffError};
use crate::boot::handoff::types::MemoryMapEntry;

use super::helpers::{baseline, baseline_with_mmap};

pub(crate) fn test_validate_security_rejects_mmap_entry_size_mismatch() -> TestResult {
    let mut h = baseline_with_mmap();
    let expected = size_of::<MemoryMapEntry>() as u32;
    h.mmap.entry_size = expected + 8;
    match validate_security(&h) {
        Err(HandoffError::MemoryMapEntrySize { expected: e, got: g }) => {
            if e == expected && g == expected + 8 {
                TestResult::Pass
            } else {
                TestResult::Fail
            }
        }
        _ => TestResult::Fail,
    }
}

pub(crate) fn test_validate_security_skips_mmap_when_ptr_zero() -> TestResult {
    let mut h = baseline();
    h.mmap.entry_size = 0;
    h.mmap.ptr = 0;
    if validate_security(&h).is_ok() {
        TestResult::Pass
    } else {
        TestResult::Fail
    }
}
