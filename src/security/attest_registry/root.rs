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

use super::format::DOMAIN;
use super::order::sorted_order;
use super::table::TABLE;

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
    let (order, used) = sorted_order(&table);

    let mut hasher = blake3::Hasher::new();
    hasher.update(DOMAIN);
    hasher.update(&(used as u32).to_be_bytes());
    for &i in &order[..used] {
        hasher.update(&table.entries[i].digest_input());
    }
    *hasher.finalize().as_bytes()
}

/// How many capsules the registry currently holds.
pub fn attested_count() -> usize {
    TABLE.lock().used
}
