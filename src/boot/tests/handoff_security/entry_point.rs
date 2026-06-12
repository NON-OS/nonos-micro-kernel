// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors

use super::super::test_result::TestResult;
use crate::boot::handoff::api::{validate_security, HandoffError};
use crate::memory::layout::constants::KERNEL_BASE;

use super::helpers::baseline;

const MIN_LOADER_ENTRY: u64 = 0x10_0000;

pub(crate) fn test_validate_security_rejects_entry_point_below_base() -> TestResult {
    let mut h = baseline();
    h.entry_point = MIN_LOADER_ENTRY - 1;
    match validate_security(&h) {
        Err(HandoffError::EntryPointOutOfRange) => TestResult::Pass,
        _ => TestResult::Fail,
    }
}

pub(crate) fn test_validate_security_rejects_entry_point_above_window() -> TestResult {
    let mut h = baseline();
    h.entry_point = KERNEL_BASE + 0x1000_0000;
    match validate_security(&h) {
        Err(HandoffError::EntryPointOutOfRange) => TestResult::Pass,
        _ => TestResult::Fail,
    }
}

pub(crate) fn test_validate_security_accepts_entry_point_at_base() -> TestResult {
    let mut h = baseline();
    h.entry_point = KERNEL_BASE;
    if validate_security(&h).is_ok() {
        TestResult::Pass
    } else {
        TestResult::Fail
    }
}

pub(crate) fn test_validate_security_accepts_low_half_loader_entry() -> TestResult {
    let mut h = baseline();
    h.entry_point = MIN_LOADER_ENTRY;
    if validate_security(&h).is_ok() {
        TestResult::Pass
    } else {
        TestResult::Fail
    }
}
