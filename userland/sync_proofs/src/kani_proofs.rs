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

//! Kani harnesses: the semaphore and seqlock properties hold for every input,
//! not just the sampled ones.

use crate::semaphore::pure as sem;
use crate::seqlock::pure as sq;

// A release never carries the count past the cap and never loses a permit, for
// every count within the invariant and every cap.
#[kani::proof]
fn release_is_bounded_by_cap() {
    let count: usize = kani::any();
    let cap: usize = kani::any();
    kani::assume(count <= cap);
    let next = sem::release_count(count, cap);
    assert!(next <= cap);
    assert!(next >= count);
}

// An acquire from a non-empty semaphore lowers the count by exactly one and
// never underflows.
#[kani::proof]
fn acquire_never_underflows() {
    let count: usize = kani::any();
    kani::assume(count > 0);
    assert!(sem::can_acquire(count));
    let after = sem::acquire_count(count);
    assert!(after < count);
    assert_eq!(after + 1, count);
}

// An accepted read was stable on entry and did not change: no torn read is
// ever accepted, for every pair of sequence samples.
#[kani::proof]
fn accepted_read_was_stable_and_unchanged() {
    let before: u32 = kani::any();
    let after: u32 = kani::any();
    if sq::read_valid(before, after) {
        assert!(sq::is_stable(before));
        assert_eq!(before, after);
    }
}

// A write pair drives the sequence odd then back to even, for every stable
// starting sequence that does not wrap.
#[kani::proof]
fn a_write_pair_returns_to_stable() {
    let start: u32 = kani::any();
    kani::assume(sq::is_stable(start));
    kani::assume(start < u32::MAX - 1);
    let mid = sq::bump(start);
    let end = sq::bump(mid);
    assert!(!sq::is_stable(mid));
    assert!(sq::is_stable(end));
}
