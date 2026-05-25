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

use crate::memory::addr::VirtAddr;

use super::{cached::CachedImage, state::ImageCache};

impl ImageCache {
    pub fn get(&self, id: usize) -> Option<&CachedImage> { self.images.get(&id) }
    pub fn get_mut(&mut self, id: usize) -> Option<&mut CachedImage> { self.images.get_mut(&id) }
    pub fn get_by_name(&self, name: &str) -> Option<&CachedImage> { self.name_index.get(name).and_then(|id| self.images.get(id)) }
    pub fn get_by_name_mut(&mut self, name: &str) -> Option<&mut CachedImage> { self.name_index.get(name).copied().and_then(|id| self.images.get_mut(&id)) }
    pub fn get_by_addr(&self, addr: VirtAddr) -> Option<&CachedImage> { self.addr_index.get(&addr.as_u64()).and_then(|id| self.images.get(id)) }
    pub fn contains(&self, name: &str) -> bool { self.name_index.contains_key(name) }
    pub fn contains_addr(&self, addr: VirtAddr) -> bool { self.addr_index.contains_key(&addr.as_u64()) }
    pub fn count(&self) -> usize { self.images.len() }
    pub fn referenced_count(&self) -> usize { self.images.values().filter(|image| image.is_referenced()).count() }
    pub fn unreferenced_count(&self) -> usize { self.images.values().filter(|image| !image.is_referenced()).count() }
    pub fn iter(&self) -> impl Iterator<Item = &CachedImage> { self.images.values() }
}
