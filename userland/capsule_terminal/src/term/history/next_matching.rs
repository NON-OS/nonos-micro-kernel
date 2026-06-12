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
    // Walk towards newer entries from the cursor and return the first one that
    // starts with `prefix`. When none remain, clear the cursor and return an
    // empty slice so the caller can restore the line being typed. Returns None
    // only when no history navigation is in progress.
    pub fn next_matching(&mut self, prefix: &[u8]) -> Option<&[u8]> {
        let mut i = self.cursor?;
        while i + 1 < self.count {
            i += 1;
            let len = self.lengths[i];
            if self.entries[i][..len].starts_with(prefix) {
                self.cursor = Some(i);
                return Some(&self.entries[i][..len]);
            }
        }
        self.cursor = None;
        Some(&[])
    }
}
