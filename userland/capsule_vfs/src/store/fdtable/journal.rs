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

use alloc::string::String;
use alloc::vec::Vec;

use super::types::Store;

// Ring depth for the access journal. Bounded so a long-running session cannot
// grow the store's footprint without limit.
const JOURNAL_CAP: usize = 256;

impl Store {
    // Record that `path` was accessed now. A path already in the ring is moved
    // to the head rather than duplicated, so a Recents view never lists the
    // same file twice.
    pub fn journal_touch(&mut self, path: &str) {
        let now = super::time::now_ms();
        if let Some(idx) = self.journal.iter().position(|(_, p)| p == path) {
            self.journal.remove(idx);
        }
        self.journal.insert(0, (now, String::from(path)));
        self.journal.truncate(JOURNAL_CAP);
    }

    // Most recently accessed paths, newest first, capped at `max`.
    pub fn journal_list(&self, max: usize) -> Vec<(u64, &str)> {
        self.journal
            .iter()
            .take(max.min(JOURNAL_CAP))
            .map(|(t, p)| (*t, p.as_str()))
            .collect()
    }
}
