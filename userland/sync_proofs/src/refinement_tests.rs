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

//! Differential proofs: the real semaphore and seqlock arithmetic, included via
//! `#[path]`, run against the executable spec over sampled inputs. Any drift
//! from the properties the Lean models state breaks the build.

use crate::semaphore::pure as sem;
use crate::seqlock::pure as sq;
use crate::spec;

// Semaphore: verification/lean Nonos/Semaphore.lean.

#[test]
fn release_agrees_with_spec_and_never_exceeds_cap() {
    for cap in 0..300usize {
        for count in 0..=cap {
            let next = sem::release_count(count, cap);
            assert_eq!(next, spec::sem_release(count, cap));
            assert!(next <= cap, "release must not exceed the cap");
            assert!(next >= count, "release must not lose a permit");
        }
    }
}

#[test]
fn acquire_lowers_the_count_by_one() {
    for count in 1..200_000usize {
        assert!(sem::can_acquire(count));
        assert_eq!(sem::acquire_count(count), spec::sem_acquire(count));
        assert_eq!(sem::acquire_count(count) + 1, count);
    }
}

#[test]
fn an_empty_semaphore_cannot_be_acquired() {
    assert!(!sem::can_acquire(0));
    assert_eq!(sem::can_acquire(0), spec::sem_can_acquire(0));
}

// Seqlock: verification/lean Nonos/Seqlock.lean.

#[test]
fn read_valid_agrees_with_spec() {
    for before in 0..1500u32 {
        for after in 0..1500u32 {
            assert_eq!(sq::read_valid(before, after), spec::seq_read_valid(before, after));
        }
    }
}

#[test]
fn a_write_pair_goes_odd_then_even() {
    for start in (0..200_000u32).step_by(2) {
        assert!(sq::is_stable(start));
        let mid = sq::bump(start);
        let end = sq::bump(mid);
        assert!(!sq::is_stable(mid), "the sequence is odd while a write is in flight");
        assert!(sq::is_stable(end), "the sequence is even once the write completes");
    }
}

#[test]
fn a_torn_read_is_always_rejected() {
    for before in 0..1500u32 {
        for after in 0..1500u32 {
            if before % 2 == 1 || before != after {
                assert!(!sq::read_valid(before, after), "an in-progress or changed read must be rejected");
            }
        }
    }
}
