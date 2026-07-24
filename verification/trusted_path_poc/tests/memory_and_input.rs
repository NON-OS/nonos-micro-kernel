// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! #400 the fixed allocator hands out no aliased frame and has no fallback
//! source; #401 draining is strictly narrower than posting, so an interrupt-only
//! driver can post input but never drain it.

use nonos_trusted_path_poc::{frame_alloc, input_authority};

#[test]
fn fixed_allocator_returns_none_on_exhaustion() {
    assert_eq!(frame_alloc::fixed_alloc(None), None);
}

#[test]
fn old_allocator_handed_out_an_aliasing_bump_frame() {
    assert_eq!(frame_alloc::old_alloc(None, 16), Some(16));
    assert!(frame_alloc::bump_aliases(&[16, 32, 48], 16, 0));
    assert_eq!(frame_alloc::fixed_alloc(None), None);
}

#[test]
fn interrupt_only_driver_can_post_but_never_drains() {
    let driver = input_authority::Rights { input_source: false, irq: true, admin: false };
    assert!(input_authority::can_post(driver), "driver may post input");
    assert!(!input_authority::can_drain(driver), "driver must not drain the input ring");
}

#[test]
fn draining_implies_posting_for_all_rights() {
    for &s in &[false, true] {
        for &i in &[false, true] {
            for &a in &[false, true] {
                let r = input_authority::Rights { input_source: s, irq: i, admin: a };
                if input_authority::can_drain(r) {
                    assert!(input_authority::can_post(r));
                }
            }
        }
    }
}
