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

use crate::buddy::constants::helpers::{buddy_address, order_to_size, size_to_order};
use crate::buddy::constants::orders::{MAX_ORDER, MIN_ORDER};
use crate::buddy::constants::sizes::MAX_BLOCK_SIZE;
use crate::phys::bitmap::index::{bit_mask, byte_of};
use crate::quota::limits::has_at_least;
use crate::region::overlap::{contains, overlaps};
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
