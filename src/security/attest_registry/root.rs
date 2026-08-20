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

use super::entry::AttestedCapsule;
use super::table::{MAX_ATTESTED, TABLE};

/// A digest over everything currently running that passed the spawn gate.
///
/// This is the value an attestation binds, and it is what makes the claim
/// "what is on this machine right now" rather than "what booted". Entries are
/// sorted by pid before folding, because removal reorders the table and a
/// verifier must reach the same digest from the same set.
///
/// An empty registry hashes the count alone rather than returning zeroes, so
/// "nothing is running" is a statement a verifier can check rather than an
/// absent value it has to interpret.
pub fn registry_root() -> [u8; 32] {
    let table = TABLE.lock();
    let mut order = [0usize; MAX_ATTESTED];
    for (i, slot) in order.iter_mut().enumerate().take(table.used) {
        *slot = i;
    }
    sort_by_pid(&mut order[..table.used], &table.entries);

    let mut hasher = blake3::Hasher::new();
    hasher.update(b"nonos.attest.registry.v1");
    hasher.update(&(table.used as u32).to_be_bytes());
    for &i in &order[..table.used] {
        hasher.update(&table.entries[i].digest_input());
    }
    *hasher.finalize().as_bytes()
}

/// Insertion sort over indices. The table is small and this runs only when an
/// attestation is produced, never on the spawn path.
fn sort_by_pid(order: &mut [usize], entries: &[AttestedCapsule; MAX_ATTESTED]) {
    let mut i = 1;
    while i < order.len() {
        let mut j = i;
        while j > 0 && entries[order[j - 1]].pid > entries[order[j]].pid {
            order.swap(j - 1, j);
            j -= 1;
        }
        i += 1;
    }
}

/// How many capsules the registry currently holds.
pub fn attested_count() -> usize {
    TABLE.lock().used
}
