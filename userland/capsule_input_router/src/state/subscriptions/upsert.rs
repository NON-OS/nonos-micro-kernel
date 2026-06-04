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

use super::{Subscription, SubscriptionTable};

impl SubscriptionTable {
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
}
