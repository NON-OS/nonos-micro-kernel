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

#[derive(Debug, Clone)]
pub struct StackLayout {
    pub stack_top: VirtAddr,
    pub stack_bottom: VirtAddr,
    pub stack_pointer: VirtAddr,
    pub argc_ptr: VirtAddr,
    pub argv_ptr: VirtAddr,
    pub envp_ptr: VirtAddr,
    pub auxv_ptr: VirtAddr,
}

impl StackLayout {
    pub fn stack_size(&self) -> usize {
        match usize::try_from(self.stack_top.as_u64() - self.stack_bottom.as_u64()) { Ok(value) => value, Err(_) => usize::MAX }
    }

    pub fn used_size(&self) -> usize {
        match usize::try_from(self.stack_top.as_u64() - self.stack_pointer.as_u64()) { Ok(value) => value, Err(_) => usize::MAX }
    }

    pub fn available_size(&self) -> usize {
        match usize::try_from(self.stack_pointer.as_u64() - self.stack_bottom.as_u64()) { Ok(value) => value, Err(_) => usize::MAX }
    }
}
