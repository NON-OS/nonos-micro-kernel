// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors

use super::super::test_result::TestResult;
use crate::boot::handoff::api::{validate_security, FbGeometryReason, HandoffError};
use crate::boot::handoff::types::flags;

use super::helpers::{baseline, baseline_with_fb};

fn expect_geometry(
    h: &crate::boot::handoff::types::BootHandoffV1,
    want: FbGeometryReason,
) -> TestResult {
    match validate_security(h) {
        Err(HandoffError::FramebufferGeometry { reason }) if reason == want => TestResult::Pass,
        _ => TestResult::Fail,
    }
}

pub(crate) fn test_validate_security_rejects_fb_zero_width() -> TestResult {
    let mut h = baseline_with_fb();
    h.fb.width = 0;
    expect_geometry(&h, FbGeometryReason::ZeroWidth)
}

pub(crate) fn test_validate_security_rejects_fb_zero_height() -> TestResult {
    let mut h = baseline_with_fb();
    h.fb.height = 0;
    expect_geometry(&h, FbGeometryReason::ZeroHeight)
}

pub(crate) fn test_validate_security_rejects_fb_zero_stride() -> TestResult {
    let mut h = baseline_with_fb();
    h.fb.stride = 0;
    expect_geometry(&h, FbGeometryReason::ZeroStride)
}

pub(crate) fn test_validate_security_rejects_fb_stride_too_small() -> TestResult {
    let mut h = baseline_with_fb();
    h.fb.stride = h.fb.width * h.fb.bytes_per_pixel() - 1;
    expect_geometry(&h, FbGeometryReason::StrideTooSmall)
}

pub(crate) fn test_validate_security_rejects_fb_area_exceeds_size() -> TestResult {
    let mut h = baseline_with_fb();
    h.fb.size = (h.fb.stride as u64) * (h.fb.height as u64) - 1;
    expect_geometry(&h, FbGeometryReason::AreaOverflow)
}

pub(crate) fn test_validate_security_skips_fb_when_unavailable() -> TestResult {
    let mut h = baseline();
    h.flags &= !flags::FB_AVAILABLE;
    h.fb.width = 0;
    h.fb.height = 0;
    h.fb.stride = 0;
    if validate_security(&h).is_ok() {
        TestResult::Pass
    } else {
        TestResult::Fail
    }
}
