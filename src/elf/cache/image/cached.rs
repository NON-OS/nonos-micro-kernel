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

use alloc::string::String;

use crate::elf::loader::ElfImage;
use crate::memory::addr::VirtAddr;

use super::id::next_cache_id;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheEntryState {
    Loading,
    Ready,
    Failed,
    Unloading,
}

#[derive(Debug)]
pub struct CachedImage {
    pub id: usize,
    pub name: String,
    pub image: ElfImage,
    pub ref_count: usize,
    pub state: CacheEntryState,
    pub load_time: u64,
}

impl CachedImage {
    pub fn new(name: String, image: ElfImage) -> Self {
        Self {
            id: next_cache_id(),
            name,
            image,
            ref_count: 1,
            state: CacheEntryState::Ready,
            load_time: 0,
        }
    }
    pub fn with_load_time(mut self, time: u64) -> Self {
        self.load_time = time;
        self
    }
    pub fn acquire(&mut self) {
        self.ref_count += 1;
    }
    pub fn release(&mut self) -> bool {
        if self.ref_count > 0 {
            self.ref_count -= 1;
        }
        self.ref_count == 0
    }
    pub fn is_referenced(&self) -> bool {
        self.ref_count > 0
    }
    pub fn base_addr(&self) -> VirtAddr {
        self.image.base_addr
    }
    pub fn entry_point(&self) -> VirtAddr {
        self.image.entry_point
    }
}
