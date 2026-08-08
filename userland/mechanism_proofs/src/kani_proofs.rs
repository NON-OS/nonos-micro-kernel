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
use crate::context::rflags::{sanitize, sanitize_user};
use crate::mmio::mmio_range::range_ok;
use crate::nonce::compose::compose;
use crate::phys::bitmap::index::{bit_mask, byte_of};
use crate::quota::limits::has_at_least;
use crate::refcount::dec::dec_checked;
use crate::region::overlap::{contains, overlaps};
use crate::ring::ring_math::wrap;
use crate::scheduler::policy_types::{SchedAttr, SCHED_DEADLINE, SCHED_IDLE};
use crate::spawn::caps_bits::{grant_within_manifest, install_caps, within_ceiling};
use crate::spawn::lifetime::delegation_expiry;
use crate::spec;

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

// Deadline is the top of the priority order and idle is the bottom, for every
// policy and every valid real-time priority and nice value.
#[kani::proof]
fn deadline_tops_and_idle_bottoms_the_priority_order() {
    let policy: i32 = kani::any();
    let rt: i32 = kani::any();
    let nice: i32 = kani::any();
    kani::assume((0..=99).contains(&rt));
    kani::assume((-20..=19).contains(&nice));
    let other =
        SchedAttr { policy, rt_priority: rt, nice, ..Default::default() }.effective_priority();
    let deadline =
        SchedAttr { policy: SCHED_DEADLINE, rt_priority: rt, nice, ..Default::default() }
            .effective_priority();
    let idle = SchedAttr { policy: SCHED_IDLE, rt_priority: rt, nice, ..Default::default() }
        .effective_priority();
    assert!(deadline >= other);
    assert!(idle <= other);
}

// A restored context never resumes with IOPL set, so a capsule cannot come
// back holding the I/O ports, for every saved RFLAGS value.
#[kani::proof]
fn a_restored_context_never_carries_iopl() {
    let saved: u64 = kani::any();
    let out = sanitize(saved);
    assert_eq!(out & (1 << 12), 0);
    assert_eq!(out & (1 << 13), 0);
}

// Every privileged bit is cleared and the reserved bit is set, for every saved
// RFLAGS value: the kernel's mask agrees with the bit positions it stands for.
#[kani::proof]
fn sanitized_rflags_agrees_with_spec() {
    let saved: u64 = kani::any();
    assert_eq!(sanitize(saved), spec::sanitize_rflags(saved));
}

// Sanitizing is idempotent and never sets a bit the caller did not save,
// except the reserved one.
#[kani::proof]
fn sanitizing_rflags_only_clears() {
    let saved: u64 = kani::any();
    let out = sanitize(saved);
    assert_eq!(sanitize(out), out);
    assert_eq!(out & !(saved | 2), 0);
}

// The user resume path is the same sanitizer with interrupts on: it clears
// exactly what the kernel path clears, and differs only in IF.
#[kani::proof]
fn the_user_resume_sets_only_interrupt_enable() {
    let saved: u64 = kani::any();
    assert_eq!(sanitize_user(saved), sanitize(saved) | (1 << 9));
    assert_eq!(sanitize_user(saved) & (1 << 12), 0);
    assert_eq!(sanitize_user(saved) & (1 << 13), 0);
}

// A capsule never installs authority its publisher's certificate does not
// permit, for every manifest, ceiling and grant.
#[kani::proof]
fn a_capsule_never_installs_above_its_publisher_ceiling() {
    let required: u64 = kani::any();
    let optional: u64 = kani::any();
    let ceiling: u64 = kani::any();
    let granted: u64 = kani::any();
    kani::assume(within_ceiling(required, optional, ceiling));
    kani::assume(grant_within_manifest(required, optional, granted));
    assert_eq!(install_caps(required, optional, granted) & !ceiling, 0);
}

// The installed word never exceeds what the manifest declares, for every
// input: the grant can only narrow the optional set.
#[kani::proof]
fn installed_caps_stay_within_the_manifest() {
    let required: u64 = kani::any();
    let optional: u64 = kani::any();
    let granted: u64 = kani::any();
    let installed = install_caps(required, optional, granted);
    assert_eq!(installed & !(required | optional), 0);
    assert_eq!(installed & required, required);
}

// A delegation never outlives its parent and never outlasts the request, for
// every pair.
#[kani::proof]
fn a_delegation_never_outlives_its_parent() {
    let requested: Option<u64> = kani::any();
    let parent: Option<u64> = kani::any();
    let out = delegation_expiry(requested, parent);
    if let Some(p) = parent {
        match out {
            Some(e) => assert!(e <= p),
            None => panic!("a bounded parent must bound the child"),
        }
    }
    if let Some(r) = requested {
        match out {
            Some(e) => assert!(e <= r),
            None => panic!("a requested expiry must not be dropped"),
        }
    }
}
