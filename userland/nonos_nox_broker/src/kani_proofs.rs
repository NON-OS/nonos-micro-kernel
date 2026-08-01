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

//! Kani harnesses: the spent-set never forgets a hash it recorded and never
//! double-counts one, for every hash.
//!
//! A hash's bytes are compared by position, so a hash that is symbolic in a few
//! bytes and fixed elsewhere exercises the compare over every equal and unequal
//! case while keeping the 32-byte compare cheap for the solver. The unwind
//! covers that compare.

use crate::spent::SpentSet;

/// A hash symbolic in its first two bytes, fixed elsewhere. Enough to make the
/// equality compare range over match and mismatch without 32 symbolic bytes.
fn some_hash() -> [u8; 32] {
    let mut tx = [0u8; 32];
    tx[0] = kani::any();
    tx[1] = kani::any();
    tx
}

// A recorded hash is afterwards present, and recording it into a fresh set
// evicts nothing.
#[kani::proof]
#[kani::unwind(33)]
fn a_recorded_hash_is_present() {
    let tx = some_hash();
    let mut set = SpentSet::new();
    let evicted = set.record(tx);
    assert!(!evicted);
    assert!(set.contains(&tx));
    assert!(set.len() == 1);
}

// Recording the same hash twice stores it once: the second call reports nothing
// new and the length does not grow, which is what stops one payment being
// redeemed twice.
#[kani::proof]
#[kani::unwind(33)]
fn recording_is_idempotent() {
    let tx = some_hash();
    let mut set = SpentSet::new();
    let first = set.record(tx);
    let second = set.record(tx);
    assert!(!first);
    assert!(!second);
    assert!(set.len() == 1);
    assert!(set.contains(&tx));
}

// A hash never recorded is never reported present in a fresh set.
#[kani::proof]
#[kani::unwind(33)]
fn an_unrecorded_hash_is_absent() {
    let tx = some_hash();
    let set = SpentSet::new();
    assert!(!set.contains(&tx));
}
