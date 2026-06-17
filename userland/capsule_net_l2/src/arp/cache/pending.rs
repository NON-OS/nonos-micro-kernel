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

use super::cache_type::Cache;
use super::constants::PENDING_CAP;

impl Cache {
    pub fn note_pending(&mut self, ipv4: [u8; 4]) {
        if self.pending.iter().any(|p| *p == Some(ipv4)) {
            return;
        }
        self.pending[self.pending_head] = Some(ipv4);
        self.pending_head = (self.pending_head + 1) % PENDING_CAP;
    }

    pub fn is_pending(&mut self, ipv4: [u8; 4]) -> bool {
        if let Some(slot) = self.pending.iter_mut().find(|p| **p == Some(ipv4)) {
            *slot = None;
            return true;
        }
        false
    }
}
