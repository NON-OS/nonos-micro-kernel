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

//! The set of transaction hashes already redeemed for a grant.
//!
//! One payment must yield one entitlement, so the broker remembers every
//! funding hash it has honoured and refuses a repeat. The store is a
//! fixed-capacity ring with no allocation, which the capsule seals to disk and
//! reloads across boots. When it fills, the oldest entry is overwritten: a very
//! old transaction could in principle be redeemed twice, but only after
//! `SPENT_CAPACITY` newer payments have pushed it out, and the capsule sizes
//! the ring so that horizon is far longer than any receipt stays queryable.
//! `record` reports whether it evicted, so the capsule can widen the store
//! rather than let the horizon shrink silently.

/// How many redeemed hashes the set holds before it wraps. The wrap and
/// contains logic does not depend on this, so under Kani it shrinks to keep the
/// symbolic state small while proving the same properties.
#[cfg(not(kani))]
pub const SPENT_CAPACITY: usize = 4096;
#[cfg(kani)]
pub const SPENT_CAPACITY: usize = 2;

/// Fixed-capacity record of redeemed funding hashes.
pub struct SpentSet {
    hashes: [[u8; 32]; SPENT_CAPACITY],
    /// Number of slots filled, saturating at `SPENT_CAPACITY`.
    len: usize,
    /// Next slot to write once full; rotates.
    next: usize,
}

impl Default for SpentSet {
    fn default() -> Self {
        Self::new()
    }
}

impl SpentSet {
    pub const fn new() -> Self {
        SpentSet { hashes: [[0u8; 32]; SPENT_CAPACITY], len: 0, next: 0 }
    }

    /// Whether `tx` has already been redeemed.
    pub fn contains(&self, tx: &[u8; 32]) -> bool {
        self.hashes[..self.len].iter().any(|h| h == tx)
    }

    /// Record `tx` as redeemed. Returns true if a previous entry was evicted to
    /// make room, so the caller can tell the ring is at capacity. A hash
    /// already present is not stored again and never evicts.
    pub fn record(&mut self, tx: [u8; 32]) -> bool {
        if self.contains(&tx) {
            return false;
        }
        let evicted = self.len == SPENT_CAPACITY;
        self.hashes[self.next] = tx;
        self.next = (self.next + 1) % SPENT_CAPACITY;
        if self.len < SPENT_CAPACITY {
            self.len += 1;
        }
        evicted
    }

    /// Filled slots, for the capsule's persistence and health reporting.
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}
