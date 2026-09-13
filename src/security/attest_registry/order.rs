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

//! The order the registry is folded in, defined once.
//!
//! Two callers need it and they must agree exactly. [`registry_root`] folds
//! entries in this order to produce the digest an attestation signs, and
//! [`registry_entries`] emits the same entries in the same order so a verifier
//! can redo that fold and reach the same value.
//!
//! Written twice, they would drift, and the failure is quiet in the worst way:
//! both halves keep working, receipts keep being produced, and every one of
//! them fails to verify with nothing to point at. So there is one function.
//!
//! [`registry_root`]: super::registry_root
//! [`registry_entries`]: super::registry_entries

use super::table::{Table, MAX_ATTESTED};

/// Indices into the table, sorted by pid, and how many are live.
///
/// Sorted rather than left in insertion order because removal is a swap-remove:
/// the same set of capsules reaches a different table layout depending on what
/// exited, and a verifier holding the set has no way to reconstruct that
/// history. Pid order is a property of the set alone.
pub(super) fn sorted_order(table: &Table) -> ([usize; MAX_ATTESTED], usize) {
    let mut order = [0usize; MAX_ATTESTED];
    for (i, slot) in order.iter_mut().enumerate().take(table.used) {
        *slot = i;
    }
    insertion_sort(&mut order[..table.used], table);
    (order, table.used)
}

/// Insertion sort over indices. The table is bounded at 256 and this runs only
/// when an attestation is produced, never on the spawn path.
fn insertion_sort(order: &mut [usize], table: &Table) {
    let mut i = 1;
    while i < order.len() {
        let mut j = i;
        while j > 0 && table.entries[order[j - 1]].pid > table.entries[order[j]].pid {
            order.swap(j - 1, j);
            j -= 1;
        }
        i += 1;
    }
}
