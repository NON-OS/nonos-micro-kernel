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

use super::metadata::FirmwareMetadata;
use crate::firmware::types::FirmwareType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatabaseResult {
    Success,
    NotFound,
    AlreadyExists,
    DatabaseFull,
    InvalidEntry,
}
#[derive(Debug, Clone)]
pub struct DatabaseEntry {
    pub firmware_type: FirmwareType,
    pub metadata: FirmwareMetadata,
    data_ptr: u64,
    data_size: u32,
}
#[derive(Debug, Clone)]
pub struct FirmwareDatabase {
    pub entries: [Option<DatabaseEntry>; 256],
    count: usize,
}
impl FirmwareDatabase {
    pub const fn new() -> Self {
        Self { entries: [const { None }; 256], count: 0 }
    }
    pub fn get_count(&self) -> usize {
        self.count
    }
    pub fn get_capacity(&self) -> usize {
        self.entries.len()
    }
    pub fn is_full(&self) -> bool {
        self.count >= self.entries.len()
    }
    pub fn contains(&self, ft: FirmwareType) -> bool {
        lookup_firmware(self, ft).is_some()
    }
    pub fn get_firmware_data(&self, ft: FirmwareType) -> Option<&[u8]> {
        let e = lookup_firmware(self, ft)?;
        unsafe { Some(core::slice::from_raw_parts(e.data_ptr as *const u8, e.data_size as usize)) }
    }
    pub fn get_metadata(&self, ft: FirmwareType) -> Option<&FirmwareMetadata> {
        lookup_firmware(self, ft).map(|e| &e.metadata)
    }
}

pub fn register_firmware(
    db: &mut FirmwareDatabase,
    ft: FirmwareType,
    m: FirmwareMetadata,
    data: &[u8],
) -> DatabaseResult {
    if db.count >= db.entries.len() {
        return DatabaseResult::DatabaseFull;
    }
    if lookup_firmware(db, ft).is_some() {
        return DatabaseResult::AlreadyExists;
    }
    if data.is_empty() {
        return DatabaseResult::InvalidEntry;
    }
    let e = DatabaseEntry {
        firmware_type: ft,
        metadata: m,
        data_ptr: data.as_ptr() as u64,
        data_size: data.len() as u32,
    };
    for s in &mut db.entries {
        if s.is_none() {
            *s = Some(e);
            db.count += 1;
            return DatabaseResult::Success;
        }
    }
    DatabaseResult::DatabaseFull
}

pub fn lookup_firmware(db: &FirmwareDatabase, ft: FirmwareType) -> Option<&DatabaseEntry> {
    db.entries.iter().filter_map(|e| e.as_ref()).find(|e| e.firmware_type == ft)
}
