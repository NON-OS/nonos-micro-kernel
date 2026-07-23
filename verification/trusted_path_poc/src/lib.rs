// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// Runnable proof-of-concept for the trusted-path audit fixes (PR #405). Each
// module mirrors the exact decision the kernel change makes; the tests assert
// the bug is closed and, where it clarifies the fix, that the old behaviour was
// exploitable. These run on the host via `cargo test`, next to the Lean proofs.

//! One module per fix. The models are small and mirror the cited source so the
//! runnable check and the Lean theorem are about the same decision.

/// #400 physical frame allocator: `src/memory/frame_alloc/types/ops.rs`.
/// The old allocator fell back to a bump range the `phys` bitmap already owned,
/// so once the bitmap filled it re-handed live frames. The fix makes `phys` the
/// sole source: exhaustion returns `None`, no fallback.
pub mod frame_alloc {
    /// Fixed allocator: whatever the bitmap reports is the whole answer.
    pub fn fixed_alloc(free: Option<u64>) -> Option<u64> {
        free
    }

    /// Old allocator: on an empty bitmap it fell through to a bump frame that
    /// overlapped the bitmap's own pool.
    pub fn old_alloc(free: Option<u64>, bump: u64) -> Option<u64> {
        free.or(Some(bump))
    }

    /// A bump frame can collide with an already-allocated frame (aliasing).
    pub fn bump_aliases(used: &[u64], base: u64, counter: u64) -> bool {
        used.contains(&(base + counter))
    }
}

/// #401 input authority: `src/capabilities/token/types/authority_broker.rs`,
/// gated in `src/syscall/contract/cap_table/mk.rs`. Posting accepts an
/// interrupt owner; draining must not, or any driver could keylog.
pub mod input_authority {
    #[derive(Clone, Copy)]
    pub struct Rights {
        pub input_source: bool,
        pub irq: bool,
        pub admin: bool,
    }

    /// `can_input_source`: may POST an input event.
    pub fn can_post(r: Rights) -> bool {
        r.input_source || r.irq || r.admin
    }

    /// `can_input_consumer`: may DRAIN/WAIT. No bare interrupt owner.
    pub fn can_drain(r: Rights) -> bool {
        r.input_source || r.admin
    }
}

/// #402 IPC reply correlation: `recv_reply_correlated` in
/// `src/syscall/microkernel/ipc/recv.rs`. Deliver only the message whose
/// correlation equals the nonzero call token; `sys_ipc_send` hardcodes 0.
pub mod reply_correlation {
    /// Scan the inbox, returning the first message matching the token.
    pub fn first_match(call_tok: u64, inbox: &[u64]) -> Option<u64> {
        inbox.iter().copied().find(|&corr| corr == call_tok)
    }
}

/// #403 service registration: `caller_can_register` in
/// `src/services/registry/auth/caller_can_register.rs`. The old rule let any
/// `pid <= 64` register without the right; the fix removes the shortcut.
pub mod register_auth {
    /// `caller`: `None` is kernel context, `Some(pid)` a capsule.
    pub fn can_register_old(caller: Option<u32>, has_right: bool) -> bool {
        match caller {
            None => true,
            Some(pid) => pid <= 64 || has_right,
        }
    }

    pub fn can_register_new(caller: Option<u32>, has_right: bool) -> bool {
        match caller {
            None => true,
            Some(_) => has_right,
        }
    }
}

/// #404 present-blit channel order:
/// `src/syscall/dispatch/router/graphics_present/blit.rs`. Surfaces store
/// B,G,R,A. RGB firmware needs the red/blue swap once; the bug ran it twice.
pub mod framebuffer {
    /// Exchange byte 0 and byte 2 of a 4-byte pixel.
    pub fn swap02(p: [u8; 4]) -> [u8; 4] {
        [p[2], p[1], p[0], p[3]]
    }

    /// The correct path: swap once for RGB, not at all for BGR.
    pub fn present(bgr: bool, p: [u8; 4]) -> [u8; 4] {
        if bgr {
            p
        } else {
            swap02(p)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- #400 frame allocator: no aliasing, no fallback source ----
    #[test]
    fn fixed_allocator_returns_none_on_exhaustion() {
        // With phys as the sole source, an empty bitmap yields None.
        assert_eq!(frame_alloc::fixed_alloc(None), None);
    }

    #[test]
    fn old_allocator_handed_out_an_aliasing_bump_frame() {
        // The bug: on exhaustion the old path returned a bump frame...
        assert_eq!(frame_alloc::old_alloc(None, 16), Some(16));
        // ...and that frame could already be live, i.e. a double-alloc.
        assert!(frame_alloc::bump_aliases(&[16, 32, 48], 16, 0));
        // The fixed allocator has no such frame to hand out.
        assert_eq!(frame_alloc::fixed_alloc(None), None);
    }

    // ---- #401 keylogging: Irq can post, cannot drain ----
    #[test]
    fn interrupt_only_driver_can_post_but_never_drains() {
        let driver = input_authority::Rights {
            input_source: false,
            irq: true,
            admin: false,
        };
        assert!(input_authority::can_post(driver), "driver may post input");
        assert!(
            !input_authority::can_drain(driver),
            "driver must NOT drain the input ring"
        );
    }

    #[test]
    fn draining_implies_posting_for_all_rights() {
        for &s in &[false, true] {
            for &i in &[false, true] {
                for &a in &[false, true] {
                    let r = input_authority::Rights {
                        input_source: s,
                        irq: i,
                        admin: a,
                    };
                    if input_authority::can_drain(r) {
                        assert!(input_authority::can_post(r));
                    }
                }
            }
        }
    }

    // ---- #402 reply injection: forged (corr 0) never matches a nonzero token ----
    #[test]
    fn forged_reply_is_never_delivered() {
        let call_tok = 0x9f3a_u64; // nonzero, from next_call_token
        let forged_inbox = [0u64, 0, 0]; // sys_ipc_send hardcodes correlation 0
        assert_eq!(reply_correlation::first_match(call_tok, &forged_inbox), None);
    }

    #[test]
    fn genuine_reply_is_delivered_and_forgeries_skipped() {
        let call_tok = 0x9f3a_u64;
        // Forged frames ahead of the genuine reply are dropped, not returned.
        let inbox = [0u64, 0, call_tok];
        assert_eq!(
            reply_correlation::first_match(call_tok, &inbox),
            Some(call_tok)
        );
    }

    // ---- #403 service squat: low pid no longer bypasses the right ----
    #[test]
    fn low_pid_bypass_is_closed() {
        // Old rule: a low pid could register with no right (the bug).
        assert!(register_auth::can_register_old(Some(1), false));
        // Fixed rule: denied for every pid without the right.
        for pid in [1u32, 2, 63, 64, 65, 1000] {
            assert!(!register_auth::can_register_new(Some(pid), false));
        }
        // Kernel context and holders of the right are still allowed.
        assert!(register_auth::can_register_new(None, false));
        assert!(register_auth::can_register_new(Some(1000), true));
    }

    // ---- #404 RGB: swap runs exactly once ----
    #[test]
    fn double_swap_was_a_noop_single_swap_converts() {
        let stored = [0xB0u8, 0x11, 0x22, 0xA0]; // stored B,G,R,A
        // The bug: swapping twice cancels, leaving RGB unconverted.
        assert_eq!(framebuffer::swap02(framebuffer::swap02(stored)), stored);
        // The fix: RGB firmware gets exactly one swap -> R,G,B,A scanout.
        assert_eq!(framebuffer::present(false, stored), [0x22, 0x11, 0xB0, 0xA0]);
        // BGR firmware takes no swap.
        assert_eq!(framebuffer::present(true, stored), stored);
    }
}
