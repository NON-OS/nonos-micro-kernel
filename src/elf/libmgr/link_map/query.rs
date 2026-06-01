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

use super::{entry::LinkMapEntry, state::LinkMap};

impl LinkMap {
    pub fn find(&self, base_addr: VirtAddr) -> Option<&LinkMapEntry> { self.entries.iter().find(|entry| entry.l_addr == base_addr.as_u64()).map(|entry| entry.as_ref()) }
    pub fn head(&self) -> *mut LinkMapEntry { self.head }
    pub fn count(&self) -> usize { self.entries.len() }
    pub fn iter(&self) -> impl Iterator<Item = &LinkMapEntry> { self.entries.iter().map(|entry| entry.as_ref()) }
}
