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
use crate::state::Entry;

impl Clipboard {
    pub fn copy(&mut self, content_type: u32, data: &[u8], now_ms: u64) {
        self.items.push_front(Entry { content_type, data: data.to_vec() });
        self.total_bytes += data.len();
        while self.items.len() > self.max_depth || self.total_bytes > self.max_total_bytes {
            if let Some(tail) = self.items.pop_back() {
                self.total_bytes = self.total_bytes.saturating_sub(tail.len());
            } else {
                break;
            }
        }
        self.last_activity_ms = now_ms;
    }
    pub fn latest_of_type(&self, content_type: u32) -> Option<&Entry> {
        self.items.iter().find(|e| e.content_type == content_type)
    }
    pub fn get_by_index(&self, index: usize) -> Option<&Entry> {
        self.items.get(index)
    }
    pub fn len(&self) -> usize {
        self.items.len()
    }
    pub fn iter(&self) -> impl Iterator<Item = &Entry> {
        self.items.iter()
    }
    pub fn clear(&mut self) {
        self.items.clear();
        self.total_bytes = 0;
    }
}
