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

use crate::elf::errors::ElfResult;

use super::{
    invoke::{invoke_addr, invoke_init_array, invoke_preinit_array},
    state::InitArrayRunner,
    validate::{validate_addr, validate_array, validate_preinit_array},
};

impl InitArrayRunner {
    pub fn run_all(&self) -> ElfResult<usize> {
        Ok(self.run_preinit_array()? + self.run_init_fn()? + self.run_init_array()?)
    }
    pub fn run_preinit_array(&self) -> ElfResult<usize> {
        let Some(info) = self.preinit_array.as_ref() else { return Ok(0) };
        validate_preinit_array(info)?;
        unsafe { invoke_preinit_array(info) };
        Ok(info.count())
    }
    pub fn run_init_fn(&self) -> ElfResult<usize> {
        let Some(addr) = self.init_fn else { return Ok(0) };
        validate_addr(addr.as_u64())?;
        unsafe { invoke_addr(addr) };
        Ok(1)
    }
    pub fn run_init_array(&self) -> ElfResult<usize> {
        let Some(info) = self.init_array.as_ref() else { return Ok(0) };
        validate_array(info)?;
        unsafe { invoke_init_array(info) };
        Ok(info.count())
    }
    pub fn total_init_count(&self) -> usize {
        self.preinit_array.map(|info| info.count()).unwrap_or(0)
            + usize::from(self.init_fn.is_some())
            + self.init_array.map(|info| info.count()).unwrap_or(0)
    }
}
