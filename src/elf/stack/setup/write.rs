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
use crate::elf::errors::ElfResult;
use crate::elf::stack::layout::{StackConfig, POINTER_SIZE};
use crate::memory::addr::VirtAddr;
use alloc::vec::Vec;

impl StackSetup {
    pub(super) fn write_strings(
        &mut self,
        config: &StackConfig,
    ) -> ElfResult<(Vec<VirtAddr>, Vec<VirtAddr>)> {
        let mut argv_ptrs = Vec::with_capacity(config.args.len());
        let mut envp_ptrs = Vec::with_capacity(config.env.len());
        for arg in &config.args {
            argv_ptrs.push(self.push_string(arg)?);
        }
        for env in &config.env {
            envp_ptrs.push(self.push_string(env)?);
        }
        Ok((argv_ptrs, envp_ptrs))
    }

    pub(super) fn write_pointers(
        &mut self,
        config: &StackConfig,
        string_ptrs: &(Vec<VirtAddr>, Vec<VirtAddr>),
    ) -> ElfResult<(VirtAddr, VirtAddr, VirtAddr)> {
        let (argv_ptrs, envp_ptrs) = string_ptrs;
        self.align_to(POINTER_SIZE);
        let auxv_ptr = self.push_auxv(&config.auxv)?;
        let envp_ptr = self.push_pointer_array(envp_ptrs)?;
        let argv_ptr = self.push_pointer_array(argv_ptrs)?;
        Ok((argv_ptr, envp_ptr, auxv_ptr))
    }

    pub(super) fn write_argc(&mut self, argc: usize) -> ElfResult<VirtAddr> {
        self.push_u64(argc as u64)
    }
}
