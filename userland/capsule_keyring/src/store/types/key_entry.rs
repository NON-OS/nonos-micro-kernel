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
use alloc::vec::Vec;

use super::key_type::KeyType;

pub(in crate::store) struct KeyEntry {
    pub(in crate::store) key_type: KeyType,
    pub(in crate::store) data: Vec<u8>,
    pub(in crate::store) owner_pid: u32,
    pub(in crate::store) created_at: u64,
    pub(in crate::store) expires_at: u64,
    pub(in crate::store) use_count: u64,
    pub(in crate::store) locked: bool,
}
