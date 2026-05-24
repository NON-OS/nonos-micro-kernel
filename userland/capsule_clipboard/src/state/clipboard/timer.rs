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

use super::types::Clipboard;

impl Clipboard {
    pub fn touch(&mut self, now_ms: u64) {
        self.last_activity_ms = now_ms;
    }
    pub fn idle_for(&self, now_ms: u64) -> u64 {
        now_ms.saturating_sub(self.last_activity_ms)
    }
    pub fn expire_if_idle(&mut self, now_ms: u64) -> bool {
        if self.idle_timeout_ms == 0 || self.items.is_empty() {
            return false;
        }
        if self.idle_for(now_ms) >= self.idle_timeout_ms {
            self.clear();
            return true;
        }
        false
    }
    pub fn set_idle_timeout_ms(&mut self, value: u64) {
        self.idle_timeout_ms = value;
    }
}
