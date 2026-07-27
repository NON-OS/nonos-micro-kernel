// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! #402 a forged reply (correlation 0) never matches a nonzero call token and
//! forgeries ahead of the genuine reply are skipped; #403 a low pid no longer
//! bypasses the right; #404 the channel swap runs exactly once.

use nonos_trusted_path_poc::{framebuffer, register_auth, reply_correlation};

#[test]
fn forged_reply_is_never_delivered() {
    let call_tok = 0x9f3a_u64;
    assert_eq!(reply_correlation::first_match(call_tok, &[0, 0, 0]), None);
}

#[test]
fn genuine_reply_is_delivered_and_forgeries_skipped() {
    let call_tok = 0x9f3a_u64;
    assert_eq!(reply_correlation::first_match(call_tok, &[0, 0, call_tok]), Some(call_tok));
}

#[test]
fn low_pid_bypass_is_closed() {
    assert!(register_auth::can_register_old(Some(1), false));
    for pid in [1u32, 2, 63, 64, 65, 1000] {
        assert!(!register_auth::can_register_new(Some(pid), false));
    }
    assert!(register_auth::can_register_new(None, false));
    assert!(register_auth::can_register_new(Some(1000), true));
}

#[test]
fn double_swap_was_a_noop_single_swap_converts() {
    let stored = [0xB0u8, 0x11, 0x22, 0xA0];
    assert_eq!(framebuffer::swap02(framebuffer::swap02(stored)), stored);
    assert_eq!(framebuffer::present(false, stored), [0x22, 0x11, 0xB0, 0xA0]);
    assert_eq!(framebuffer::present(true, stored), stored);
}
