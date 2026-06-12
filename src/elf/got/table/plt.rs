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

use super::{constants::PLT_ENTRY_SIZE, state::GlobalOffsetTable};

impl GlobalOffsetTable {
    pub fn plt_entry_address(&self, plt_index: usize) -> Option<VirtAddr> {
        self.plt_base
            .filter(|_| plt_index < self.plt_entry_count)
            .map(|base| VirtAddr::new(base.as_u64() + (plt_index * PLT_ENTRY_SIZE) as u64))
    }

    pub fn got_entry_for_plt(&self, plt_index: usize) -> Option<usize> {
        (plt_index < self.plt_entry_count).then_some(plt_index + 3)
    }
}
