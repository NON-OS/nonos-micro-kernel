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

use core::sync::atomic::AtomicU32;

pub const ENTRY_CAP: usize = 128;
pub const NAME_BYTES: usize = 64;

#[derive(Clone, Copy)]
pub struct CacheEntry {
    pub name_hash: u64,
    pub name: [u8; NAME_BYTES],
    pub name_len: u8,
    pub ipv4: [u8; 4],
    pub expires_ms: u64,
}

pub struct Cache {
    pub(super) entries: [Option<CacheEntry>; ENTRY_CAP],
    pub(super) epoch: AtomicU32,
}

impl Cache {
    pub const fn new() -> Self {
        Self { entries: [None; ENTRY_CAP], epoch: AtomicU32::new(0) }
    }
}
