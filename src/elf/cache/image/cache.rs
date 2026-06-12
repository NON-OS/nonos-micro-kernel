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

use alloc::{string::String, vec::Vec};

use crate::elf::errors::{ElfError, ElfResult};
use crate::elf::loader::ElfImage;

use super::{cached::CachedImage, state::ImageCache};

impl ImageCache {
    pub fn insert(&mut self, name: String, image: ElfImage) -> ElfResult<usize> {
        if self.name_index.contains_key(&name) {
            return Err(ElfError::LibraryAlreadyLoaded);
        }
        if self.images.len() >= self.max_entries {
            self.evict_unreferenced()?;
        }
        let cached = CachedImage::new(name.clone(), image);
        let id = cached.id;
        self.name_index.insert(name, id);
        self.addr_index.insert(cached.base_addr().as_u64(), id);
        self.images.insert(id, cached);
        Ok(id)
    }

    pub fn acquire(&mut self, id: usize) -> ElfResult<()> {
        self.images.get_mut(&id).ok_or(ElfError::LibraryNotFound)?.acquire();
        Ok(())
    }
    pub fn acquire_by_name(&mut self, name: &str) -> ElfResult<usize> {
        let id = *self.name_index.get(name).ok_or(ElfError::LibraryNotFound)?;
        self.acquire(id)?;
        Ok(id)
    }
    pub fn release(&mut self, id: usize) -> ElfResult<bool> {
        Ok(self.images.get_mut(&id).ok_or(ElfError::LibraryNotFound)?.release())
    }

    pub fn remove(&mut self, id: usize) -> ElfResult<CachedImage> {
        let cached = self.images.remove(&id).ok_or(ElfError::LibraryNotFound)?;
        self.name_index.remove(&cached.name);
        self.addr_index.remove(&cached.base_addr().as_u64());
        Ok(cached)
    }

    pub fn remove_if_unreferenced(&mut self, id: usize) -> ElfResult<Option<CachedImage>> {
        if self.images.get(&id).is_some_and(|cached| !cached.is_referenced()) {
            self.remove(id).map(Some)
        } else {
            Ok(None)
        }
    }

    fn evict_unreferenced(&mut self) -> ElfResult<()> {
        let ids: Vec<usize> = self
            .images
            .iter()
            .filter(|(_, image)| !image.is_referenced())
            .map(|(&id, _)| id)
            .collect();
        if ids.is_empty() {
            return Err(ElfError::CacheFull);
        }
        self.remove(ids[0]).map(|_| ())
    }

    pub fn clear_unreferenced(&mut self) -> usize {
        let ids: Vec<usize> = self
            .images
            .iter()
            .filter(|(_, image)| !image.is_referenced())
            .map(|(&id, _)| id)
            .collect();
        let mut cleared = 0;
        for id in ids {
            if self.remove(id).is_ok() {
                cleared += 1;
            }
        }
        cleared
    }
}
