// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors

use super::super::test_result::TestResult;
use super::{entropy, entry_point, framebuffer, helpers, memory_map};

type Test = (&'static str, fn() -> TestResult);

const TESTS: &[Test] = &[
    ("accepts_baseline", helpers::test_validate_security_accepts_baseline),
    ("rejects_zero_seed", entropy::test_validate_security_rejects_zero_seed),
    (
        "rejects_mmap_entry_size_mismatch",
        memory_map::test_validate_security_rejects_mmap_entry_size_mismatch,
    ),
    ("skips_mmap_when_ptr_zero", memory_map::test_validate_security_skips_mmap_when_ptr_zero),
    ("rejects_fb_zero_width", framebuffer::test_validate_security_rejects_fb_zero_width),
    ("rejects_fb_zero_height", framebuffer::test_validate_security_rejects_fb_zero_height),
    ("rejects_fb_zero_stride", framebuffer::test_validate_security_rejects_fb_zero_stride),
    (
        "rejects_fb_stride_too_small",
        framebuffer::test_validate_security_rejects_fb_stride_too_small,
    ),
    (
        "rejects_fb_area_exceeds_size",
        framebuffer::test_validate_security_rejects_fb_area_exceeds_size,
    ),
    ("skips_fb_when_unavailable", framebuffer::test_validate_security_skips_fb_when_unavailable),
    (
        "rejects_entry_point_below_base",
        entry_point::test_validate_security_rejects_entry_point_below_base,
    ),
    (
        "rejects_entry_point_above_window",
        entry_point::test_validate_security_rejects_entry_point_above_window,
    ),
    (
        "accepts_entry_point_at_base",
        entry_point::test_validate_security_accepts_entry_point_at_base,
    ),
    (
        "accepts_low_half_loader_entry",
        entry_point::test_validate_security_accepts_low_half_loader_entry,
    ),
];

pub fn all_pass() -> bool {
    TESTS.iter().all(|(_, f)| f().passed())
}

pub fn run_each() -> &'static [Test] {
    TESTS
}
