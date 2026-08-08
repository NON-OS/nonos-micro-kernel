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
    /// The most recent line that begins with `prefix`, without moving the
    /// recall cursor.
    ///
    /// Separate from `prev_matching` because this runs on every keystroke to
    /// draw a suggestion, and recall is something the reader asks for with an
    /// arrow key. Sharing a cursor between the two would make typing walk the
    /// history, so the next arrow press would resume from somewhere the
    /// reader never chose.
    ///
    /// An empty prefix suggests nothing. Every line matches it, so the newest
    /// one would sit there permanently and mean nothing.
    pub fn suggest(&self, prefix: &[u8]) -> Option<&[u8]> {
        if prefix.is_empty() {
            return None;
        }
        let mut i = self.count;
        while i > 0 {
            i -= 1;
            let len = self.lengths[i];
            let entry = &self.entries[i][..len];
            // A line identical to what is typed suggests nothing to add, and
            // drawing an empty tail would put the accept key on a no-op.
            if entry.len() > prefix.len() && entry.starts_with(prefix) {
                return Some(entry);
            }
        }
        None
    }
}
