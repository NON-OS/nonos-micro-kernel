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

use alloc::{collections::BTreeMap, string::String};

use super::cached::CachedImage;

pub struct ImageCache {
    pub(super) images: BTreeMap<usize, CachedImage>,
    pub(super) name_index: BTreeMap<String, usize>,
    pub(super) addr_index: BTreeMap<u64, usize>,
    pub(super) max_entries: usize,
}

impl ImageCache {
    pub fn new() -> Self {
        Self::with_capacity(256)
    }
    pub fn with_capacity(max_entries: usize) -> Self {
        Self {
            images: BTreeMap::new(),
            name_index: BTreeMap::new(),
            addr_index: BTreeMap::new(),
            max_entries,
        }
    }

    pub fn clear(&mut self) {
        self.images.clear();
        self.name_index.clear();
        self.addr_index.clear();
    }
}

impl Default for ImageCache {
    fn default() -> Self {
        Self::new()
    }
}
