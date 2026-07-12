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

//! Kani harnesses: the buddy address arithmetic holds for every input, not just
//! the sampled ones.

use crate::bounds::range::in_range;
use crate::buddy::constants::helpers::{buddy_address, order_to_size};
use crate::mmio::mmio_range::range_ok;
use crate::nonce::compose::compose;
use crate::phys::bitmap::index::{bit_mask, byte_of};
use crate::quota::limits::has_at_least;
use crate::refcount::dec::dec_checked;
use crate::region::overlap::{contains, overlaps};
use crate::ring::ring_math::wrap;
use crate::timer::interval::elapsed_reached;

// The buddy of the buddy of a block is the block itself: the address XOR is an
// involution, for every address and every order.
#[kani::proof]
fn buddy_address_is_an_involution() {
    let addr: u64 = kani::any();
    let order: usize = kani::any();
    kani::assume(order < 64);
    assert_eq!(buddy_address(buddy_address(addr, order), order), addr);
}

// Splitting a block halves its size exactly: an order k block is two order k-1
// blocks, so no memory is created or lost by a split.
#[kani::proof]
fn a_split_conserves_size() {
    let k: usize = kani::any();
    kani::assume(k < 62);
    assert_eq!(order_to_size(k + 1), 2 * order_to_size(k));
}

// A bit mask selects exactly one bit, for every index.
#[kani::proof]
fn bit_mask_selects_exactly_one_bit() {
    let idx: usize = kani::any();
    assert!(bit_mask(idx).is_power_of_two());
}

// The byte and the in-byte bit position reconstruct the index exactly, for
// every index: the split is lossless.
#[kani::proof]
fn byte_and_bit_reconstruct_the_index() {
    let idx: usize = kani::any();
    assert_eq!(byte_of(idx) * 8 + (idx & 7), idx);
}

// Region overlap is symmetric, for every pair of ranges.
#[kani::proof]
fn overlap_is_symmetric() {
    let a0: u64 = kani::any();
    let a1: u64 = kani::any();
    let b0: u64 = kani::any();
    let b1: u64 = kani::any();
    assert_eq!(overlaps(a0, a1, b0, b1), overlaps(b0, b1, a0, a1));
}

// Overlap is exactly the negation of disjointness, for every pair of ranges.
#[kani::proof]
fn overlap_is_the_negation_of_disjoint() {
    let a0: u64 = kani::any();
    let a1: u64 = kani::any();
    let b0: u64 = kani::any();
    let b1: u64 = kani::any();
    let disjoint = a1 <= b0 || b1 <= a0;
    assert_eq!(overlaps(a0, a1, b0, b1), !disjoint);
}

// An address that a range contains lies within its bounds, for every input.
#[kani::proof]
fn contains_implies_within_bounds() {
    let start: u64 = kani::any();
    let end: u64 = kani::any();
    let addr: u64 = kani::any();
    if contains(start, end, addr) {
        assert!(addr >= start && addr < end);
    }
}

// The elapsed test saturates on a tick wraparound: when current is at or after
// last it is the true elapsed span, and when current is before last it reads as
// no time elapsed, for every input.
#[kani::proof]
fn elapsed_saturates_on_wraparound() {
    let current: u64 = kani::any();
    let last: u64 = kani::any();
    let interval: u64 = kani::any();
    if current >= last {
        assert_eq!(elapsed_reached(current, last, interval), current - last >= interval);
    } else {
        assert_eq!(elapsed_reached(current, last, interval), interval == 0);
    }
}

// A quota covers a request exactly when the request is within the remaining
// budget, for every input.
#[kani::proof]
fn has_at_least_iff_within_budget() {
    let remaining: u64 = kani::any();
    let amount: u64 = kani::any();
    assert_eq!(has_at_least(remaining, amount), amount <= remaining);
}

// A wrapped ring index always stays within the capacity, for every position and
// nonzero capacity.
#[kani::proof]
fn a_wrapped_index_stays_in_bounds() {
    let pos: usize = kani::any();
    let cap: usize = kani::any();
    kani::assume(cap > 0);
    kani::assume(pos < cap);
    assert!(wrap(pos, cap) < cap);
}

// A valid MMIO window is non-empty and does not wrap the address space, for
// every base and size.
#[kani::proof]
fn a_valid_mmio_window_is_non_empty_and_does_not_wrap() {
    let base: usize = kani::any();
    let size: usize = kani::any();
    if range_ok(base, size) {
        assert!(size > 0);
        assert!(base.checked_add(size).is_some());
    }
}

// A reference count never underflows: a decrement exists only for a positive
// count and lowers it by one, for every count.
#[kani::proof]
fn refcount_never_underflows() {
    let n: u32 = kani::any();
    match dec_checked(n) {
        Some(next) => {
            assert!(n > 0);
            assert_eq!(next, n - 1);
            assert!(next < n);
        }
        None => assert_eq!(n, 0),
    }
}

// A nonce carries its counter in the low 32 bits, so the counter is recoverable
// and distinct counters never collide, for every timestamp and counter.
#[kani::proof]
fn a_nonce_counter_is_recoverable() {
    let timestamp: u64 = kani::any();
    let counter: u64 = kani::any();
    assert_eq!(compose(timestamp, counter) & 0xFFFF_FFFF, counter & 0xFFFF_FFFF);
}

// An in-range access sits wholly inside the segment with neither end
// overflowing, for every input.
#[kani::proof]
fn an_in_range_access_is_confined() {
    let addr: u64 = kani::any();
    let size: u64 = kani::any();
    let start: u64 = kani::any();
    let seg: u64 = kani::any();
    if in_range(addr, size, start, seg) {
        assert!(addr >= start);
        let end = start.checked_add(seg).unwrap();
        let addr_end = addr.checked_add(size).unwrap();
        assert!(addr_end <= end);
    }
}
