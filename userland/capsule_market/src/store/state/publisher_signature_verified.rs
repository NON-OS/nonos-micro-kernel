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

use super::Accepted;

impl Accepted {
    pub fn publisher_signature_verified(&self, entry_index: usize, release_index: usize) -> bool {
        let mut flat_index = release_index;
        for entry in self.index.entries.iter().take(entry_index) {
            flat_index = flat_index.saturating_add(entry.releases.len());
        }
        match self.publisher_signature_verified.get(flat_index) {
            Some(v) => *v,
            None => false,
        }
    }
}
