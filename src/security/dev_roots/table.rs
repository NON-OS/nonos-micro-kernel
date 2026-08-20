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

/// Few on purpose. A machine with dozens of signing authorities has no
/// meaningful notion of who vouched for what, and enrolment is a deliberate
/// act, not something that should accumulate quietly.
pub const MAX_DEV_ROOTS: usize = 4;

pub(super) struct DevRoot {
    pub root: [u8; 32],
    pub used: bool,
}

pub(super) struct Table {
    pub roots: [DevRoot; MAX_DEV_ROOTS],
}

impl Table {
    const fn new() -> Self {
        const EMPTY: DevRoot = DevRoot { root: [0u8; 32], used: false };
        Self { roots: [EMPTY; MAX_DEV_ROOTS] }
    }

    /// Returns the slot, or None when full or already present. Enrolling the
    /// same key twice is not an error worth failing a session over, but it
    /// must not consume a second slot.
    pub fn insert(&mut self, root: [u8; 32]) -> Option<u8> {
        for (i, slot) in self.roots.iter().enumerate() {
            if slot.used && slot.root == root {
                return Some(i as u8);
            }
        }
        for (i, slot) in self.roots.iter_mut().enumerate() {
            if !slot.used {
                slot.root = root;
                slot.used = true;
                return Some(i as u8);
            }
        }
        None
    }

    /// Checked before a challenge is shown, so a user is never asked to
    /// approve an enrolment that cannot be stored.
    pub fn is_full(&self) -> bool {
        self.roots.iter().all(|s| s.used)
    }

    pub fn find(&self, root: &[u8; 32]) -> Option<u8> {
        self.roots
            .iter()
            .position(|s| s.used && &s.root == root)
            .map(|i| i as u8)
    }
}

pub(super) static TABLE: Mutex<Table> = Mutex::new(Table::new());
