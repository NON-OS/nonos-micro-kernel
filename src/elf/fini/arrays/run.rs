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

use super::{invoke::{invoke_addr, invoke_array}, state::FiniArrayRunner, validate::{validate_addr, validate_array}};

impl FiniArrayRunner {
    pub fn run_all(&self) -> ElfResult<usize> { Ok(self.run_fini_array()? + self.run_fini_fn()?) }
    pub fn run_fini_array(&self) -> ElfResult<usize> {
        let Some(info) = self.fini_array.as_ref() else { return Ok(0) };
        validate_array(info)?;
        unsafe { invoke_array(info) };
        Ok(info.count())
    }
    pub fn run_fini_fn(&self) -> ElfResult<usize> {
        let Some(addr) = self.fini_fn else { return Ok(0) };
        validate_addr(addr.as_u64())?;
        unsafe { invoke_addr(addr) };
        Ok(1)
    }
    pub fn total_fini_count(&self) -> usize { self.fini_array.map(|info| info.count()).unwrap_or(0) + usize::from(self.fini_fn.is_some()) }
}
