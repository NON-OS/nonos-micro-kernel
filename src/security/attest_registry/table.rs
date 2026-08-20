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

use spin::Mutex;

use super::entry::AttestedCapsule;

/// Bounded because the registry is consulted from the spawn path and must not
/// allocate there. A full table is refused loudly rather than silently
/// dropping an entry: an attestation missing a running capsule is worse than
/// no attestation, because it understates what is on the machine.
pub(super) const MAX_ATTESTED: usize = 256;

pub(super) struct Table {
    pub entries: [AttestedCapsule; MAX_ATTESTED],
    pub used: usize,
}

impl Table {
    const fn new() -> Self {
        Self { entries: [AttestedCapsule::empty(); MAX_ATTESTED], used: 0 }
    }

    pub(super) fn insert(&mut self, e: AttestedCapsule) -> bool {
        if self.used >= MAX_ATTESTED {
            return false;
        }
        self.entries[self.used] = e;
        self.used += 1;
        true
    }

    /// Swap-remove: order is not preserved, so the root is computed over a
    /// sorted view rather than over insertion order.
    pub(super) fn remove(&mut self, pid: u32) -> bool {
        let mut i = 0;
        while i < self.used {
            if self.entries[i].pid == pid {
                self.entries[i] = self.entries[self.used - 1];
                self.entries[self.used - 1] = AttestedCapsule::empty();
                self.used -= 1;
                return true;
            }
            i += 1;
        }
        false
    }
}

pub(super) static TABLE: Mutex<Table> = Mutex::new(Table::new());
