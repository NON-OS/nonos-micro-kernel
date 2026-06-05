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

impl Cache {
    pub(super) fn evict_oldest(&mut self) {
        let mut oldest_idx = None;
        let mut oldest_age = 0u32;
        for (i, slot) in self.entries.iter().enumerate() {
            if let Some(e) = slot {
                if e.age_ticks >= oldest_age {
                    oldest_age = e.age_ticks;
                    oldest_idx = Some(i);
                }
            }
        }
        if let Some(i) = oldest_idx {
            self.entries[i] = None;
            self.len -= 1;
        }
    }
}
