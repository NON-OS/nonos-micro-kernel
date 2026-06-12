// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors

use super::super::test_result::TestResult;
use crate::boot::handoff::api::{validate_security, HandoffError};

use super::helpers::baseline;

pub(crate) fn test_validate_security_rejects_zero_seed() -> TestResult {
    let mut h = baseline();
    h.rng.seed32 = [0u8; 32];
    match validate_security(&h) {
        Err(HandoffError::WeakEntropy) => TestResult::Pass,
        _ => TestResult::Fail,
    }
}
