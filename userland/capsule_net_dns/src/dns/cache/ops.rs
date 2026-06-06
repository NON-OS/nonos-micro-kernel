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

use core::sync::atomic::Ordering;

use super::entry::{Cache, CacheEntry, ENTRY_CAP, NAME_BYTES};
use super::hash::hash;

impl Cache {
    pub fn lookup(&self, name: &str, now_ms: u64) -> Option<[u8; 4]> {
        let h = hash(name);
        self.entries
            .iter()
            .flatten()
            .find_map(|e| {
                let n = e.name_len as usize;
                let same = n == name.len() && e.name[..n] == *name.as_bytes();
                (e.name_hash == h && same && now_ms < e.expires_ms).then_some(e.ipv4)
            })
    }

    pub fn insert(&mut self, name: &str, ipv4: [u8; 4], ttl_ms: u64, now_ms: u64) {
        let entry = entry_for(name, ipv4, ttl_ms, now_ms);
        if let Some(slot) = self.find_slot(&entry) {
            *slot = Some(entry);
            return;
        }
        let idx = (self.epoch.fetch_add(1, Ordering::Relaxed) as usize) % ENTRY_CAP;
        self.entries[idx] = Some(entry);
    }

    pub fn tick(&mut self, now_ms: u64) {
        for slot in self.entries.iter_mut() {
            if slot.as_ref().is_some_and(|e| now_ms >= e.expires_ms) {
                *slot = None;
            }
        }
    }

    fn find_slot(&mut self, entry: &CacheEntry) -> Option<&mut Option<CacheEntry>> {
        self.entries.iter_mut().find(|e| {
            e.as_ref().map_or(true, |x| x.name_hash == entry.name_hash && x.name == entry.name)
        })
    }
}

fn entry_for(name: &str, ipv4: [u8; 4], ttl_ms: u64, now_ms: u64) -> CacheEntry {
    let bytes = name.as_bytes();
    let name_len = bytes.len().min(NAME_BYTES);
    let mut name_buf = [0u8; NAME_BYTES];
    name_buf[..name_len].copy_from_slice(&bytes[..name_len]);
    CacheEntry {
        name_hash: hash(name),
        name: name_buf,
        name_len: name_len as u8,
        ipv4,
        expires_ms: now_ms.saturating_add(ttl_ms),
    }
}
