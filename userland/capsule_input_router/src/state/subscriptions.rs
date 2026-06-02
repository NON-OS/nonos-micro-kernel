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

pub const MAX_SUBSCRIBERS: usize = 16;

#[derive(Clone, Copy, Default)]
pub struct Subscription {
    pub pid: u32,
    pub kind_mask: u32,
    pub in_use: bool,
}

pub struct SubscriptionTable {
    entries: [Subscription; MAX_SUBSCRIBERS],
}

impl SubscriptionTable {
    pub const fn new() -> Self {
        Self {
            entries: [Subscription { pid: 0, kind_mask: 0, in_use: false }; MAX_SUBSCRIBERS],
        }
    }

    // Either upserts the (pid, kind_mask) row or, when kind_mask is
    // zero, drops the subscriber entirely.
    pub fn upsert(&mut self, pid: u32, kind_mask: u32) -> bool {
        for entry in self.entries.iter_mut() {
            if entry.in_use && entry.pid == pid {
                if kind_mask == 0 {
                    *entry = Subscription::default();
                } else {
                    entry.kind_mask = kind_mask;
                }
                return true;
            }
        }
        if kind_mask == 0 {
            return true;
        }
        for entry in self.entries.iter_mut() {
            if !entry.in_use {
                *entry = Subscription { pid, kind_mask, in_use: true };
                return true;
            }
        }
        false
    }

    pub fn match_kind(&self, kind: u16) -> impl Iterator<Item = u32> + '_ {
        let bit = 1u32.checked_shl(kind as u32).unwrap_or(0);
        self.entries
            .iter()
            .filter(move |e| e.in_use && (e.kind_mask & bit) != 0)
            .map(|e| e.pid)
    }

    pub fn allows(&self, pid: u32, kind: u16) -> bool {
        let bit = 1u32.checked_shl(kind as u32).unwrap_or(0);
        self.entries.iter().any(|e| e.in_use && e.pid == pid && (e.kind_mask & bit) != 0)
    }
}
