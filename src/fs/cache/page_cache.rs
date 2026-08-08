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

use alloc::{collections::BTreeMap, vec::Vec};
use core::sync::atomic::Ordering;
use spin::{Mutex, Once};

use super::types::{DirtyPage, CACHE_STATS, MAX_CACHED_PAGES};

static PAGE_CACHE: Once<Mutex<PageCache>> = Once::new();

#[derive(Debug, Clone)]
pub(crate) struct CachedPage {
    pub(crate) data: Vec<u8>,
    pub(crate) dirty: bool,
    pub(crate) accessed: u64,
    pub(crate) ref_count: u32,
}

pub(crate) struct PageCache {
    pages: BTreeMap<(u64, u64), CachedPage>,
    dirty_list: Vec<(u64, u64)>,
    total_cached_bytes: usize,
    lru_counter: u64,
}

impl PageCache {
    fn new() -> Self {
        Self {
            pages: BTreeMap::new(),
            dirty_list: Vec::new(),
            total_cached_bytes: 0,
            lru_counter: 0,
        }
    }

    pub(crate) fn get_page(&mut self, file_id: u64, offset: u64) -> Option<&CachedPage> {
        let key = (file_id, offset);
        if let Some(page) = self.pages.get_mut(&key) {
            self.lru_counter += 1;
            page.accessed = self.lru_counter;
            CACHE_STATS.hits.fetch_add(1, Ordering::Relaxed);
            return Some(page);
        }
        CACHE_STATS.misses.fetch_add(1, Ordering::Relaxed);
        None
    }

    pub(crate) fn insert_page(&mut self, file_id: u64, offset: u64, data: Vec<u8>, dirty: bool) {
        let key = (file_id, offset);
        self.lru_counter += 1;

        while self.pages.len() >= MAX_CACHED_PAGES {
            self.evict_lru_page();
        }

        let page =
            CachedPage { data: data.clone(), dirty, accessed: self.lru_counter, ref_count: 1 };

        self.total_cached_bytes += page.data.len();

        if dirty && !self.dirty_list.contains(&key) {
            self.dirty_list.push(key);
        }

        self.pages.insert(key, page);
    }

    pub(crate) fn mark_clean(&mut self, file_id: u64, offset: u64) {
        let key = (file_id, offset);
        if let Some(page) = self.pages.get_mut(&key) {
            page.dirty = false;
        }
        self.dirty_list.retain(|k| *k != key);
    }

    fn evict_lru_page(&mut self) {
        let mut lru_key: Option<(u64, u64)> = None;
        let mut lru_time = u64::MAX;

        for (key, page) in &self.pages {
            if !page.dirty && page.accessed < lru_time && page.ref_count == 0 {
                lru_time = page.accessed;
                lru_key = Some(*key);
            }
        }

        if lru_key.is_none() {
            for (key, page) in &self.pages {
                if page.accessed < lru_time {
                    lru_time = page.accessed;
                    lru_key = Some(*key);
                }
            }
        }

        if let Some(key) = lru_key {
            if let Some(page) = self.pages.remove(&key) {
                self.total_cached_bytes = self.total_cached_bytes.saturating_sub(page.data.len());
                self.dirty_list.retain(|k| *k != key);
                CACHE_STATS.evictions.fetch_add(1, Ordering::Relaxed);
            }
        }
    }

    pub(crate) fn clear(&mut self) {
        self.pages.clear();
        self.dirty_list.clear();
        self.total_cached_bytes = 0;
        self.lru_counter = 0;
    }

    pub(crate) fn stats(&self) -> (usize, usize, usize) {
        (self.pages.len(), self.dirty_list.len(), self.total_cached_bytes)
    }
}

pub fn init_page_cache() {
    PAGE_CACHE.call_once(|| Mutex::new(PageCache::new()));
}

pub fn get_dirty_pages() -> BTreeMap<u64, Vec<DirtyPage>> {
    init_page_cache();
    let mut result = BTreeMap::new();

    if let Some(cache) = PAGE_CACHE.get() {
        let cache_guard = cache.lock();
        for (key, page) in &cache_guard.pages {
            if page.dirty {
                let entry = result.entry(key.0).or_insert_with(Vec::new);
                entry.push(DirtyPage { offset: key.1, data: page.data.clone() });
            }
        }
    }
    result
}

pub fn mark_page_clean(file_id: u64, offset: u64) {
    init_page_cache();
    if let Some(cache) = PAGE_CACHE.get() {
        cache.lock().mark_clean(file_id, offset);
    }
}

pub fn clear_page_cache() {
    init_page_cache();
    if let Some(cache) = PAGE_CACHE.get() {
        cache.lock().clear();
    }
}

pub fn get_page_cache_stats() -> (usize, usize, usize) {
    init_page_cache();
    if let Some(cache) = PAGE_CACHE.get() {
        cache.lock().stats()
    } else {
        (0, 0, 0)
    }
}

pub fn cache_page(file_id: u64, offset: u64, data: Vec<u8>, dirty: bool) {
    init_page_cache();
    if let Some(cache) = PAGE_CACHE.get() {
        cache.lock().insert_page(file_id, offset, data, dirty);
    }
}

pub fn get_cached_page(file_id: u64, offset: u64) -> Option<Vec<u8>> {
    init_page_cache();
    if let Some(cache) = PAGE_CACHE.get() {
        cache.lock().get_page(file_id, offset).map(|p| p.data.clone())
    } else {
        None
    }
}

pub fn prefetch_pages(_file_id: u64, _offset: u64, _length: u64) -> Result<(), &'static str> {
    init_page_cache();
    Ok(())
}

pub fn invalidate_pages(file_id: u64, offset: u64, length: u64) {
    init_page_cache();
    if let Some(cache) = PAGE_CACHE.get() {
        let mut guard = cache.lock();
        let page_size = 4096u64;
        let start_page = offset / page_size;
        let end_offset = if length == u64::MAX { u64::MAX } else { offset.saturating_add(length) };
        let keys_to_remove: Vec<_> = guard
            .pages
            .keys()
            .filter(|(fid, off)| {
                *fid == file_id
                    && *off >= start_page * page_size
                    && (length == u64::MAX || *off < end_offset)
            })
            .copied()
            .collect();
        for key in keys_to_remove {
            if let Some(page) = guard.pages.remove(&key) {
                guard.total_cached_bytes = guard.total_cached_bytes.saturating_sub(page.data.len());
                guard.dirty_list.retain(|k| *k != key);
            }
        }
    }
}
