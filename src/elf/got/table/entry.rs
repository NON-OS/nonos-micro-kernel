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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GotEntryType {
    Null,
    Dynamic,
    PltResolver,
    LinkMap,
    Symbol(usize),
}

#[derive(Debug, Clone)]
pub struct GotEntry {
    pub index: usize,
    pub address: VirtAddr,
    pub value: u64,
    pub entry_type: GotEntryType,
    pub resolved: bool,
}

impl GotEntry {
    pub fn new(index: usize, address: VirtAddr, value: u64, entry_type: GotEntryType) -> Self {
        Self { index, address, value, entry_type, resolved: false }
    }

    pub fn resolve(&mut self, target: u64) {
        self.value = target;
        self.resolved = true;
    }
}
