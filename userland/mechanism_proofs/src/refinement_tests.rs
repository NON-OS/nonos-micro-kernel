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

//! Differential proofs: the real buddy arithmetic, included via `#[path]`, run
//! against the executable spec over sampled inputs.

use crate::bounds::range::in_range;
use crate::buddy::constants::helpers::{buddy_address, order_to_size, size_to_order};
use crate::buddy::constants::orders::{MAX_ORDER, MIN_ORDER};
use crate::buddy::constants::sizes::MAX_BLOCK_SIZE;
use crate::context::rflags::{sanitize, sanitize_user};
use crate::mmio::mmio_range::range_ok;
use crate::nonce::compose::compose;
use crate::phys::bitmap::index::{bit_mask, byte_of};
use crate::quota::limits::has_at_least;
use crate::refcount::dec::dec_checked;
use crate::region::overlap::{contains, overlaps};
use crate::ring::ring_math::{is_full, wrap};
use crate::scheduler::policy_types::{
    SchedAttr, SCHED_BATCH, SCHED_DEADLINE, SCHED_FIFO, SCHED_IDLE, SCHED_NORMAL, SCHED_RR,
};
use crate::spawn::caps_bits::{grant_within_manifest, install_caps, within_ceiling};
use crate::spawn::lifetime::delegation_expiry;
use crate::spec;

fn sched_attr(policy: i32, rt_priority: i32, nice: i32) -> SchedAttr {
    SchedAttr { policy, rt_priority, nice, ..Default::default() }
}

// Buddy allocator: verification/lean Nonos/Buddy.lean.

#[test]
fn order_to_size_is_the_power_of_two() {
    for k in 0..40usize {
        assert_eq!(order_to_size(k), 1usize << k);
        assert_eq!(order_to_size(k), spec::buddy_order_size(k));
        assert!(order_to_size(k).is_power_of_two());
    }
}

#[test]
fn splitting_conserves_size() {
    for k in 1..40usize {
        assert_eq!(order_to_size(k), 2 * order_to_size(k - 1));
    }
}

#[test]
fn a_buddy_of_a_buddy_is_the_block_itself() {
    for a in (0..200_000u64).step_by(7) {
        for o in 0..30usize {
            assert_eq!(buddy_address(buddy_address(a, o), o), a);
        }
    }
}

#[test]
fn size_to_order_covers_the_request_and_stays_in_range() {
    for size in 1..MAX_BLOCK_SIZE {
        let order = size_to_order(size);
        assert!((MIN_ORDER..=MAX_ORDER).contains(&order));
        // the chosen block is large enough to hold the request
        assert!(order_to_size(order) >= size || order == MAX_ORDER);
    }
}

// Bitmap allocator: verification/lean Nonos/Bitmap.lean.

#[test]
fn byte_and_mask_agree_with_spec() {
    for idx in 0..200_000usize {
        assert_eq!(byte_of(idx), spec::bitmap_byte_of(idx));
        assert_eq!(bit_mask(idx), spec::bitmap_bit_mask(idx));
    }
}

#[test]
fn each_index_selects_exactly_one_bit_and_reconstructs() {
    for idx in 0..200_000usize {
        assert!(bit_mask(idx).is_power_of_two(), "a mask selects exactly one bit");
        assert_eq!(
            byte_of(idx) * 8 + (idx % 8),
            idx,
            "byte times eight plus bit position is the index"
        );
    }
}

#[test]
fn the_eight_bits_of_a_byte_are_distinct_and_cover_it() {
    for base in (0..80_000usize).step_by(8) {
        let mut seen = 0u8;
        for b in 0..8 {
            let m = bit_mask(base + b);
            assert_eq!(seen & m, 0, "no two bits in a byte share a mask");
            seen |= m;
        }
        assert_eq!(seen, 0xff, "the eight masks cover the whole byte");
    }
}

// Memory region range algebra: verification/lean Nonos/Interval.lean, Vma.lean.

#[test]
fn overlap_agrees_with_spec_and_is_commutative() {
    for a0 in 0..25u64 {
        for a1 in a0..25 {
            for b0 in 0..25u64 {
                for b1 in b0..25 {
                    let o = overlaps(a0, a1, b0, b1);
                    assert_eq!(o, spec::region_overlaps(a0, a1, b0, b1));
                    assert_eq!(o, overlaps(b0, b1, a0, a1), "overlap is symmetric");
                }
            }
        }
    }
}

#[test]
fn overlap_is_exactly_the_negation_of_disjoint() {
    for a0 in 0..25u64 {
        for a1 in 0..25u64 {
            for b0 in 0..25u64 {
                for b1 in 0..25u64 {
                    assert_eq!(overlaps(a0, a1, b0, b1), !spec::region_disjoint(a0, a1, b0, b1));
                }
            }
        }
    }
}

#[test]
fn contains_holds_only_within_the_range() {
    for s in 0..30u64 {
        for e in s..30 {
            for addr in 0..30u64 {
                assert_eq!(contains(s, e, addr), addr >= s && addr < e);
            }
        }
    }
}

// Resource quota check: verification/lean Nonos/Quota.lean.

#[test]
fn has_at_least_agrees_with_spec() {
    for remaining in 0..300u64 {
        for amount in 0..300u64 {
            assert_eq!(
                has_at_least(remaining, amount),
                spec::quota_has_at_least(remaining, amount)
            );
            assert_eq!(has_at_least(remaining, amount), amount <= remaining);
        }
    }
}

// Input ring index arithmetic: verification/lean Nonos/Ring.lean.

#[test]
fn wrap_agrees_with_spec_and_stays_in_bounds() {
    for cap in 1..64usize {
        for pos in 0..cap {
            let w = wrap(pos, cap);
            assert_eq!(w, spec::ring_wrap(pos, cap));
            assert!(w < cap, "a wrapped index stays within the capacity");
        }
    }
}

#[test]
fn a_full_ring_is_detected_when_the_head_would_reach_the_tail() {
    for cap in 2..64usize {
        for head in 0..cap {
            for tail in 0..cap {
                assert_eq!(is_full(head, tail, cap), wrap(head, cap) == tail);
            }
        }
    }
}

// MMIO window validity: verification/lean Nonos/Mmio.lean.

#[test]
fn range_ok_agrees_with_spec_and_rejects_empty_and_wrapping() {
    let samples: [usize; 8] =
        [0, 1, 4096, 0x1000, usize::MAX - 1, usize::MAX, usize::MAX / 2, 0x8000];
    for &base in &samples {
        for &size in &samples {
            assert_eq!(range_ok(base, size), spec::mmio_range_ok(base, size));
        }
    }
    assert!(!range_ok(0x1000, 0), "an empty window is rejected");
    assert!(!range_ok(usize::MAX, 1), "a wrapping window is rejected");
    assert!(range_ok(0x1000, 0x1000), "a normal window is accepted");
}

// Page reference count: verification/lean Nonos/Refcount.lean.

#[test]
fn dec_never_underflows_and_agrees_with_spec() {
    assert_eq!(dec_checked(0), None);
    for n in 0..200_000u32 {
        assert_eq!(dec_checked(n), spec::refcount_dec(n));
        if n > 0 {
            assert_eq!(dec_checked(n), Some(n - 1));
            assert!(dec_checked(n).unwrap() < n);
        }
    }
}

// Token nonce: verification/lean Nonos/Nonce.lean.

#[test]
fn a_nonce_carries_its_counter_in_the_low_32_bits() {
    for t in (0..2000u64).step_by(7) {
        for c in (0..200_000u64).step_by(13) {
            assert_eq!(compose(t, c) & 0xFFFF_FFFF, c & 0xFFFF_FFFF, "the counter is recoverable");
        }
    }
    assert_ne!(compose(5, 1), compose(5, 2), "distinct counters give distinct nonces");
}

// Relocation-target bounds: verification/lean Nonos/Bounds.lean.

#[test]
fn in_range_agrees_with_spec_and_confines_the_access() {
    for start in 0..20u64 {
        for seg in 0..20u64 {
            for addr in 0..25u64 {
                for size in 0..8u64 {
                    assert_eq!(
                        in_range(addr, size, start, seg),
                        spec::in_range(addr, size, start, seg)
                    );
                }
            }
        }
    }
    assert!(in_range(10, 4, 8, 8), "an access inside the segment is in range");
    assert!(!in_range(14, 4, 8, 8), "an access past the segment end is out of range");
    assert!(!in_range(u64::MAX, 2, 0, 100), "a wrapping access is out of range");
}

// Scheduling priority order: verification/lean Nonos/Priority.lean.

#[test]
fn effective_priority_agrees_with_spec() {
    let policies = [SCHED_NORMAL, SCHED_FIFO, SCHED_RR, SCHED_BATCH, SCHED_IDLE, SCHED_DEADLINE, 7];
    for &policy in &policies {
        for nice in -20..=19 {
            for rt in 0..=99 {
                assert_eq!(
                    sched_attr(policy, rt, nice).effective_priority(),
                    spec::effective_priority(policy, rt, nice)
                );
            }
        }
    }
}

#[test]
fn deadline_tops_and_idle_bottoms_the_order() {
    let policies = [SCHED_NORMAL, SCHED_FIFO, SCHED_RR, SCHED_BATCH, SCHED_IDLE, SCHED_DEADLINE];
    for nice in -20..=19 {
        for rt in 0..=99 {
            let deadline = sched_attr(SCHED_DEADLINE, rt, nice).effective_priority();
            let idle = sched_attr(SCHED_IDLE, rt, nice).effective_priority();
            for &policy in &policies {
                let p = sched_attr(policy, rt, nice).effective_priority();
                assert!(deadline >= p, "deadline is the top of the order");
                assert!(idle <= p, "idle is the bottom of the order");
            }
        }
    }
}

#[test]
fn a_realtime_task_preempts_a_timesharing_one() {
    for nice in -20..=19 {
        for rt in 0..=99 {
            let fifo = sched_attr(SCHED_FIFO, rt, nice).effective_priority();
            let rr = sched_attr(SCHED_RR, rt, nice).effective_priority();
            let normal = sched_attr(SCHED_NORMAL, rt, nice).effective_priority();
            let batch = sched_attr(SCHED_BATCH, rt, nice).effective_priority();
            assert!(fifo > normal && fifo > batch, "fifo preempts timesharing");
            assert!(rr > normal && rr > batch, "rr preempts timesharing");
        }
    }
}

// Restored RFLAGS: verification/lean Nonos/Rflags.lean.

#[test]
fn sanitized_rflags_agrees_with_the_spec() {
    // Every single-bit pattern, every pair of the flag bits that matter, and
    // the all-ones case: enough to pin the mask bit for bit.
    for bit in 0..64 {
        let saved = 1u64 << bit;
        assert_eq!(sanitize(saved), spec::sanitize_rflags(saved), "bit {bit}");
    }
    for a in 0..24 {
        for b in 0..24 {
            let saved = (1u64 << a) | (1u64 << b);
            assert_eq!(sanitize(saved), spec::sanitize_rflags(saved), "bits {a},{b}");
        }
    }
    assert_eq!(sanitize(u64::MAX), spec::sanitize_rflags(u64::MAX));
    assert_eq!(sanitize(0), spec::sanitize_rflags(0));
}

#[test]
fn a_restored_context_never_carries_a_privileged_flag() {
    // TF, DF, IOPL low and high, NT, RF, VM, AC, VIF, VIP.
    let privileged = [8, 10, 12, 13, 14, 16, 17, 18, 19, 20];
    for bit in privileged {
        let out = sanitize(u64::MAX);
        assert_eq!(out & (1 << bit), 0, "bit {bit} survived");
        assert_eq!(sanitize_user(u64::MAX) & (1 << bit), 0, "bit {bit} survived a user resume");
    }
    assert_eq!(sanitize(0) & 2, 2, "the reserved bit is restored");
}

#[test]
fn sanitizing_rflags_never_grants_a_bit() {
    for bit in 0..64 {
        let saved = 1u64 << bit;
        let out = sanitize(saved);
        assert_eq!(out & !(saved | 2), 0, "bit {bit} appeared from nowhere");
        assert_eq!(sanitize(out), out, "bit {bit} is not idempotent");
    }
}

#[test]
fn only_the_user_resume_enables_interrupts() {
    // A CPL=0 continuation saved with IF=0 has to resume with IF=0, or the
    // timer lands on a path already holding a scheduler lock.
    assert_eq!(sanitize(0) & (1 << 9), 0);
    assert_eq!(sanitize(1 << 9) & (1 << 9), 1 << 9);
    assert_eq!(sanitize_user(0) & (1 << 9), 1 << 9);
}

// Spawn capability ceiling: verification/lean Nonos/SpawnCaps.lean.

#[test]
fn the_spawn_gate_agrees_with_the_spec() {
    let vals: [u64; 9] = [0, 1, 2, 3, 0xF0, 0xFF, 0x8000_0000_0000_0000, u64::MAX, 0x0F0F_0F0F];
    for &r in &vals {
        for &o in &vals {
            for &c in &vals {
                assert_eq!(within_ceiling(r, o, c), spec::spawn_within_ceiling(r, o, c));
                assert_eq!(
                    grant_within_manifest(r, o, c),
                    spec::spawn_grant_within_manifest(r, o, c)
                );
                assert_eq!(install_caps(r, o, c), spec::spawn_install_caps(r, o, c));
            }
        }
    }
}

#[test]
fn a_capsule_never_installs_above_its_publisher_ceiling() {
    // The composition that matters: the ceiling comes from the signed NØNOS ID
    // certificate, and install_caps is what reaches the PCB.
    let vals: [u64; 8] = [0, 1, 3, 0xF0, 0xFF, 0xDEAD_BEEF, u64::MAX, 0x0F0F_0F0F];
    for &r in &vals {
        for &o in &vals {
            for &c in &vals {
                if !within_ceiling(r, o, c) {
                    continue;
                }
                for &g in &vals {
                    if !grant_within_manifest(r, o, g) {
                        continue;
                    }
                    let installed = install_caps(r, o, g);
                    assert_eq!(installed & !c, 0, "installed authority escaped the ceiling");
                }
            }
        }
    }
}

#[test]
fn required_capabilities_survive_an_empty_grant() {
    // Required caps are deliberately not attenuated by the grant, so the
    // ceiling is the only bound. Pin that rather than let it drift.
    for &r in &[0u64, 1, 0xFF, u64::MAX] {
        for &o in &[0u64, 1, 0xFF, u64::MAX] {
            assert_eq!(install_caps(r, o, 0), r);
            assert_eq!(install_caps(r, o, u64::MAX), r | o);
            assert_eq!(install_caps(r, o, 0) & !r, 0);
        }
    }
}

// Delegation lifetime: verification/lean Nonos/Delegation.lean.

#[test]
fn a_delegation_never_outlives_its_parent() {
    let vals = [None, Some(0u64), Some(1), Some(500), Some(u64::MAX)];
    for &requested in &vals {
        for &parent in &vals {
            let out = delegation_expiry(requested, parent);
            assert_eq!(out, spec::delegation_expiry(requested, parent));
            if let Some(p) = parent {
                let e = out.expect("a bounded parent always bounds the child");
                assert!(e <= p, "child outlived its parent");
            }
            if let Some(r) = requested {
                let e = out.expect("a requested expiry is never dropped");
                assert!(e <= r, "child outlasted what was asked for");
            }
        }
    }
}

#[test]
fn an_unbounded_parent_leaves_the_request_alone() {
    assert_eq!(delegation_expiry(None, None), None);
    assert_eq!(delegation_expiry(Some(7), None), Some(7));
    assert_eq!(delegation_expiry(None, Some(7)), Some(7));
    assert_eq!(delegation_expiry(Some(9), Some(7)), Some(7));
    assert_eq!(delegation_expiry(Some(3), Some(7)), Some(3));
}
