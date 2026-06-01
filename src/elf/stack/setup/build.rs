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

use super::state::StackSetup;
use crate::elf::errors::{ElfError, ElfResult};
use crate::elf::stack::layout::{StackConfig, StackLayout};
use crate::memory::addr::VirtAddr;

impl StackSetup {
    pub fn setup(&mut self, config: &StackConfig) -> ElfResult<StackLayout> {
        if config.total_setup_size() > self.available_space() {
            return Err(ElfError::MemoryAllocationFailed);
        }
        let string_ptrs = self.write_strings(config)?;
        let (argv_ptr, envp_ptr, auxv_ptr) = self.write_pointers(config, &string_ptrs)?;
        let argc_ptr = self.write_argc(config.argc())?;
        self.align_stack();
        Ok(StackLayout {
            stack_top: self.stack_top,
            stack_bottom: self.stack_bottom,
            stack_pointer: self.current,
            argc_ptr,
            argv_ptr,
            envp_ptr,
            auxv_ptr,
        })
    }
}

pub fn setup_user_stack(
    stack_top: VirtAddr,
    stack_size: usize,
    config: &StackConfig,
) -> ElfResult<StackLayout> {
    StackSetup::new(stack_top, stack_size).setup(config)
}
