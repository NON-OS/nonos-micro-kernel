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

use crate::elf::stack::layout::STACK_ALIGNMENT;
use crate::memory::addr::VirtAddr;

pub struct StackSetup {
    pub(super) stack_top: VirtAddr,
    pub(super) stack_bottom: VirtAddr,
    pub(super) current: VirtAddr,
}

impl StackSetup {
    pub fn new(stack_top: VirtAddr, stack_size: usize) -> Self {
        let stack_bottom = VirtAddr::new(stack_top.as_u64() - stack_size as u64);
        Self { stack_top, stack_bottom, current: stack_top }
    }

    pub(super) fn align_to(&mut self, alignment: usize) {
        self.current = VirtAddr::new(self.current.as_u64() & !(alignment as u64 - 1));
    }

    pub(super) fn align_stack(&mut self) {
        self.align_to(STACK_ALIGNMENT);
    }

    pub(super) fn available_space(&self) -> usize {
        match usize::try_from(self.current.as_u64() - self.stack_bottom.as_u64()) {
            Ok(value) => value,
            Err(_) => usize::MAX,
        }
    }
}
