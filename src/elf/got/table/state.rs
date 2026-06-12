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

use alloc::vec::Vec;

use crate::memory::addr::VirtAddr;

use super::{constants::GOT_ENTRY_SIZE, entry::GotEntry};

pub struct GlobalOffsetTable {
    pub(super) base: VirtAddr,
    pub(super) entry_count: usize,
    pub(super) entries: Vec<GotEntry>,
    pub(super) plt_base: Option<VirtAddr>,
    pub(super) plt_entry_count: usize,
}

impl GlobalOffsetTable {
    pub fn new(base: VirtAddr, size: usize) -> Self {
        let entry_count = size / GOT_ENTRY_SIZE;
        Self {
            base,
            entry_count,
            entries: Vec::with_capacity(entry_count),
            plt_base: None,
            plt_entry_count: 0,
        }
    }
}
