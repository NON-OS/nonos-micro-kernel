// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use crate::firmware::types::FirmwareType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheResult {
    Hit,
    Miss,
    Evicted,
    Error,
    Full,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CachePolicy {
    LeastRecentlyUsed,
    LeastFrequentlyUsed,
    FirstInFirstOut,
}
#[derive(Debug, Clone, Copy)]
struct CacheEntry {
    firmware_type: FirmwareType,
    data_ptr: u64,
    data_size: u32,
    access_count: u16,
    last_access: u32,
    valid: bool,
}
impl CacheEntry {
    const fn empty() -> Self {
        Self {
            firmware_type: FirmwareType::Unknown,
            data_ptr: 0,
            data_size: 0,
            access_count: 0,
            last_access: 0,
            valid: false,
        }
    }
}
#[derive(Debug, Clone)]
pub struct MemoryCache {
    entries: [CacheEntry; 64],
    policy: CachePolicy,
    hits: u32,
    misses: u32,
}
impl MemoryCache {
    pub const fn new(policy: CachePolicy) -> Self {
        Self { entries: [CacheEntry::empty(); 64], policy, hits: 0, misses: 0 }
    }
    pub fn get_hit_rate(&self) -> f32 {
        if self.hits + self.misses == 0 {
            0.0
        } else {
            self.hits as f32 / (self.hits + self.misses) as f32
        }
    }
    pub fn get_firmware(&mut self, ft: FirmwareType) -> Option<&[u8]> {
        for e in &mut self.entries {
            if e.valid && e.firmware_type == ft {
                e.access_count = e.access_count.saturating_add(1);
                e.last_access = ts();
                self.hits += 1;
                return Some(unsafe {
                    core::slice::from_raw_parts(e.data_ptr as *const u8, e.data_size as usize)
                });
            }
        }
        self.misses += 1;
        None
    }
}

pub fn cache_firmware(c: &mut MemoryCache, ft: FirmwareType, data: &[u8]) -> CacheResult {
    if data.len() > 16 * 1024 * 1024 {
        return CacheResult::Error;
    }
    if let Some(i) = c.entries.iter().position(|e| e.valid && e.firmware_type == ft) {
        c.entries[i] = CacheEntry {
            firmware_type: ft,
            data_ptr: data.as_ptr() as u64,
            data_size: data.len() as u32,
            access_count: 1,
            last_access: ts(),
            valid: true,
        };
        return CacheResult::Hit;
    }
    let victim = match c.policy {
        CachePolicy::LeastRecentlyUsed => c
            .entries
            .iter()
            .enumerate()
            .filter(|(_, e)| e.valid)
            .min_by_key(|(_, e)| e.last_access)
            .map(|(i, _)| i),
        CachePolicy::LeastFrequentlyUsed => c
            .entries
            .iter()
            .enumerate()
            .filter(|(_, e)| e.valid)
            .min_by_key(|(_, e)| e.access_count)
            .map(|(i, _)| i),
        CachePolicy::FirstInFirstOut => c.entries.iter().position(|e| e.valid),
    };
    if let Some(vi) = victim {
        c.entries[vi] = CacheEntry {
            firmware_type: ft,
            data_ptr: data.as_ptr() as u64,
            data_size: data.len() as u32,
            access_count: 1,
            last_access: ts(),
            valid: true,
        };
        CacheResult::Evicted
    } else {
        CacheResult::Full
    }
}

pub fn invalidate_cache(c: &mut MemoryCache, ft: FirmwareType) -> bool {
    for e in &mut c.entries {
        if e.valid && e.firmware_type == ft {
            e.valid = false;
            return true;
        }
    }
    false
}
fn ts() -> u32 {
    static mut C: u32 = 0;
    unsafe {
        C += 1;
        C
    }
}
