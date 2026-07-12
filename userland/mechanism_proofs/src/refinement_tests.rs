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
use crate::spec;

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
