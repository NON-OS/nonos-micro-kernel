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

use super::authority::Authority;
use super::table::TABLE;

/// Which authority owns this root, if any.
///
/// The vendor root is checked by the caller before this is consulted, and the
/// two sets are never merged. Keeping them separate is what stops an enrolled
/// key from ever being reported as the vendor's: a bug that collapsed them
/// would make locally built code indistinguishable from shipped code in every
/// attestation the machine produces.
pub fn authority_for(root: &[u8; 32]) -> Option<Authority> {
    TABLE.lock().find(root).map(Authority::Developer)
}

/// Every enrolled root, for a verifier that wants to try each in turn.
///
/// Returned by value rather than as a borrow of the table, so the lock is not
/// held while a caller runs proof verification against them.
pub fn enrolled_roots() -> ([[u8; 32]; super::table::MAX_DEV_ROOTS], usize) {
    let table = TABLE.lock();
    let mut out = [[0u8; 32]; super::table::MAX_DEV_ROOTS];
    let mut n = 0;
    for slot in table.roots.iter() {
        if slot.used {
            out[n] = slot.root;
            n += 1;
        }
    }
    (out, n)
}
