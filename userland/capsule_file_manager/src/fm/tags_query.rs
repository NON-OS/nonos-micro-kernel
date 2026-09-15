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

extern crate alloc;

use alloc::vec::Vec;

use super::tags::TagMap;

impl TagMap {
    pub fn tags_for(&self, path: &str) -> Vec<&str> {
        match self.entries.iter().find(|(p, _)| p == path) {
            Some((_, tags)) => tags.iter().map(|t| t.as_str()).collect(),
            None => Vec::new(),
        }
    }

    pub fn paths_with(&self, tag: &str) -> Vec<&str> {
        let tag = tag.to_ascii_lowercase();
        self.entries
            .iter()
            .filter(|(_, tags)| tags.contains(&tag))
            .map(|(p, _)| p.as_str())
            .collect()
    }

    pub fn tagged_paths(&self) -> Vec<&str> {
        self.entries.iter().map(|(p, _)| p.as_str()).collect()
    }

    pub fn drop_path(&mut self, path: &str) {
        self.entries.retain(|(p, _)| p != path);
    }
}
