// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

extern crate alloc;
use super::super::constants::{FREE_LIST_COUNT, MAX_ORDER, MIN_ORDER};
use super::super::error::BuddyAllocResult;
use super::super::types::{AllocatedBlock, BuddyBlock};
use crate::memory::layout;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

pub struct VmapAllocator {
    pub(super) free_lists: [Vec<BuddyBlock>; FREE_LIST_COUNT],
    pub(super) allocated_blocks: BTreeMap<u64, AllocatedBlock>,
    pub(super) base_addr: u64,
    pub(super) total_size: u64,
    pub(super) initialized: bool,
}

impl VmapAllocator {
    pub const fn new() -> Self {
        const INIT: Vec<BuddyBlock> = Vec::new();
        Self {
            free_lists: [INIT; FREE_LIST_COUNT],
            allocated_blocks: BTreeMap::new(),
            base_addr: layout::VMAP_BASE,
            total_size: layout::VMAP_SIZE,
            initialized: false,
        }
    }

    pub fn init(&mut self) -> BuddyAllocResult<()> {
        if self.initialized {
            return Ok(());
        }
        for list in &mut self.free_lists {
            list.clear();
        }
        self.allocated_blocks.clear();
        let block_size = 1u64 << MAX_ORDER;
        let list_idx = MAX_ORDER - MIN_ORDER;
        let region_end = self.base_addr.saturating_add(self.total_size);
        let mut addr = self.base_addr;
        while addr.saturating_add(block_size) <= region_end && list_idx < self.free_lists.len() {
            self.free_lists[list_idx].push(BuddyBlock { addr, order: MAX_ORDER });
            addr = addr.saturating_add(block_size);
        }
        self.initialized = true;
        Ok(())
    }
}
