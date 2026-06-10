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

use super::optimize::CompressionType;
use crate::firmware::types::FirmwareType;
use core::ptr::addr_of;

const STORAGE_BACKEND_SIZE: usize = 0x100000;
static mut STORAGE_BACKEND: [u8; STORAGE_BACKEND_SIZE] = [0u8; STORAGE_BACKEND_SIZE];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageResult {
    Success,
    NotFound,
    StorageFull,
    CorruptedData,
    AccessDenied,
}
#[derive(Debug, Clone)]
pub struct CacheEntry {
    firmware_type: FirmwareType,
    storage_offset: u64,
    compressed_size: u32,
    original_size: u32,
    compression_type: CompressionType,
    checksum: u32,
    valid: bool,
}
impl CacheEntry {
    const fn empty() -> Self {
        Self {
            firmware_type: FirmwareType::Unknown,
            storage_offset: 0,
            compressed_size: 0,
            original_size: 0,
            compression_type: CompressionType::None,
            checksum: 0,
            valid: false,
        }
    }
    pub fn get_compression_ratio(&self) -> f32 {
        if self.original_size == 0 {
            1.0
        } else {
            self.compressed_size as f32 / self.original_size as f32
        }
    }
    pub fn storage_offset(&self) -> u64 {
        self.storage_offset
    }
}
#[derive(Debug, Clone)]
pub struct StorageCache {
    entries: [CacheEntry; 128],
    storage_used: u64,
    storage_limit: u64,
}
impl StorageCache {
    pub const fn new(limit: u64) -> Self {
        Self {
            entries: [const { CacheEntry::empty() }; 128],
            storage_used: 0,
            storage_limit: limit,
        }
    }
    pub fn get_usage(&self) -> f32 {
        if self.storage_limit == 0 {
            0.0
        } else {
            self.storage_used as f32 / self.storage_limit as f32
        }
    }
    pub fn contains(&self, ft: FirmwareType) -> bool {
        self.entries.iter().any(|e| e.valid && e.firmware_type == ft)
    }
}

pub fn persist_cache(
    s: &mut StorageCache,
    ft: FirmwareType,
    data: &[u8],
    comp: CompressionType,
) -> StorageResult {
    if s.storage_used + data.len() as u64 > s.storage_limit {
        return StorageResult::StorageFull;
    }
    let cd = super::optimize::compress_firmware(data, comp);
    let cs = cd.iter().fold(0u32, |a, &b| a.wrapping_add(u32::from(b)));
    static mut NEXT: u64 = 1024;
    let off = unsafe {
        let o = NEXT;
        NEXT += cd.len() as u64;
        if NEXT <= s.storage_limit {
            o
        } else {
            0
        }
    };
    if off == 0 {
        return StorageResult::StorageFull;
    }
    let backend_max = STORAGE_BACKEND_SIZE;
    if (off as usize) + cd.len() > backend_max {
        return StorageResult::StorageFull;
    }
    unsafe {
        core::ptr::copy_nonoverlapping(
            cd.as_ptr(),
            addr_of!(STORAGE_BACKEND).cast_mut().cast::<u8>().add(off as usize),
            cd.len(),
        );
    }
    let entry = CacheEntry {
        firmware_type: ft,
        storage_offset: off,
        compressed_size: cd.len() as u32,
        original_size: data.len() as u32,
        compression_type: comp,
        checksum: cs,
        valid: true,
    };
    if let Some(i) = s.entries.iter().position(|e| !e.valid) {
        s.entries[i] = entry;
        s.storage_used += cd.len() as u64;
        StorageResult::Success
    } else {
        StorageResult::StorageFull
    }
}

pub fn load_cache(
    s: &StorageCache,
    ft: FirmwareType,
) -> Result<alloc::vec::Vec<u8>, StorageResult> {
    let e = s
        .entries
        .iter()
        .find(|e| e.valid && e.firmware_type == ft)
        .ok_or(StorageResult::NotFound)?;
    let off = e.storage_offset() as usize;
    let len = e.compressed_size as usize;
    let backend_max = STORAGE_BACKEND_SIZE;
    if off + len > backend_max {
        return Err(StorageResult::CorruptedData);
    }
    let mut cd: alloc::vec::Vec<u8> = alloc::vec![0u8; len];
    unsafe {
        core::ptr::copy_nonoverlapping(
            addr_of!(STORAGE_BACKEND).cast::<u8>().add(off),
            cd.as_mut_ptr(),
            len,
        );
    }
    let cs = cd.iter().fold(0u32, |a, &b| a.wrapping_add(u32::from(b)));
    if cs != e.checksum {
        return Err(StorageResult::CorruptedData);
    }
    super::optimize::decompress_firmware(&cd, e.compression_type)
        .map_err(|_| StorageResult::CorruptedData)
}
