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

use super::types::History;

impl History {
    /// The newest line at or before `before` that contains `needle`.
    ///
    /// Substring rather than prefix, because a reader searching their history
    /// usually remembers a word from the middle of a command and not how it
    /// began. Returns the position alongside the line so a caller can ask for
    /// the next match older than this one.
    ///
    /// An empty needle matches the newest line, which is what an empty search
    /// should show: the thing you would get by pressing up once.
    pub fn search_back(&self, needle: &[u8], before: usize) -> Option<(usize, &[u8])> {
        let mut i = before.min(self.count);
        while i > 0 {
            i -= 1;
            let len = self.lengths[i];
            let entry = &self.entries[i][..len];
            if contains(entry, needle) {
                return Some((i, entry));
            }
        }
        None
    }
}

/// Whether `haystack` holds `needle` anywhere in it.
fn contains(haystack: &[u8], needle: &[u8]) -> bool {
    if needle.is_empty() {
        return true;
    }
    if needle.len() > haystack.len() {
        return false;
    }
    haystack.windows(needle.len()).any(|w| w == needle)
}
