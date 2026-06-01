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

use super::constants::FINI_FN_SIZE;

#[derive(Debug, Clone, Copy)]
pub struct FiniArrayInfo {
    pub addr: VirtAddr,
    pub size: usize,
}

impl FiniArrayInfo {
    pub fn new(addr: VirtAddr, size: usize) -> Self { Self { addr, size } }
    pub fn count(&self) -> usize { self.size / FINI_FN_SIZE }
    pub fn is_empty(&self) -> bool { self.size == 0 }
    pub fn is_entry_aligned(&self) -> bool { self.size % FINI_FN_SIZE == 0 }
    pub fn end_addr(&self) -> Option<u64> { self.addr.as_u64().checked_add(self.size as u64) }
}
