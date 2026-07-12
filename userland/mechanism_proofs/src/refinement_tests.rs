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
use crate::mmio::mmio_range::range_ok;
use crate::nonce::compose::compose;
use crate::phys::bitmap::index::{bit_mask, byte_of};
use crate::quota::limits::has_at_least;
use crate::refcount::dec::dec_checked;
use crate::region::overlap::{contains, overlaps};
use crate::ring::ring_math::{is_full, wrap};
use crate::spec;
use crate::timer::interval::elapsed_reached;

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
        assert_eq!(byte_of(idx) * 8 + (idx % 8), idx, "byte times eight plus bit position is the index");
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

// Load balancer elapsed test: verification/lean Nonos/Timer.lean.

#[test]
fn elapsed_agrees_with_spec_and_saturates() {
    for current in 0..160u64 {
        for last in 0..160u64 {
            for interval in 0..160u64 {
                assert_eq!(
                    elapsed_reached(current, last, interval),
                    spec::timer_elapsed_reached(current, last, interval)
                );
            }
        }
    }
    // an earlier "current" (wraparound) counts as no time elapsed
    assert!(elapsed_reached(5, 10, 0));
    assert!(!elapsed_reached(5, 10, 1));
}

// Resource quota check: verification/lean Nonos/Quota.lean.

#[test]
fn has_at_least_agrees_with_spec() {
    for remaining in 0..300u64 {
        for amount in 0..300u64 {
            assert_eq!(has_at_least(remaining, amount), spec::quota_has_at_least(remaining, amount));
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
    let samples: [usize; 8] = [0, 1, 4096, 0x1000, usize::MAX - 1, usize::MAX, usize::MAX / 2, 0x8000];
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
                    assert_eq!(in_range(addr, size, start, seg), spec::in_range(addr, size, start, seg));
                }
            }
        }
    }
    assert!(in_range(10, 4, 8, 8), "an access inside the segment is in range");
    assert!(!in_range(14, 4, 8, 8), "an access past the segment end is out of range");
    assert!(!in_range(u64::MAX, 2, 0, 100), "a wrapping access is out of range");
}
