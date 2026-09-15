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

use alloc::string::String;

use super::tags::{valid_tag, TagMap, TAG_MAX_ASSIGNMENTS, TAG_MAX_PER_PATH};

impl TagMap {
    pub fn add(&mut self, path: &str, tag: &str) -> bool {
        let tag = tag.to_ascii_lowercase();
        if !valid_tag(&tag) {
            return false;
        }
        if let Some(slot) = self.entries.iter_mut().find(|(p, _)| p == path) {
            if slot.1.contains(&tag) {
                return true;
            }
            if slot.1.len() >= TAG_MAX_PER_PATH {
                return false;
            }
            slot.1.push(tag);
            return true;
        }
        if self.entries.len() >= TAG_MAX_ASSIGNMENTS {
            return false;
        }
        self.entries.push((String::from(path), alloc::vec![tag]));
        true
    }

    pub fn remove(&mut self, path: &str, tag: &str) {
        let tag = tag.to_ascii_lowercase();
        if let Some(idx) = self.entries.iter().position(|(p, _)| p == path) {
            self.entries[idx].1.retain(|t| *t != tag);
            if self.entries[idx].1.is_empty() {
                self.entries.remove(idx);
            }
        }
    }
}
