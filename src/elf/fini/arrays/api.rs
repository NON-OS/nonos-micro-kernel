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
use crate::memory::addr::VirtAddr;

use super::{info::FiniArrayInfo, invoke::{invoke_addr, invoke_array}, validate::{validate_addr, validate_array}};

pub fn run_fini_array(addr: VirtAddr, size: usize) -> ElfResult<usize> {
    let info = FiniArrayInfo::new(addr, size);
    validate_array(&info)?;
    unsafe { invoke_array(&info) };
    Ok(info.count())
}

pub fn call_fini_function(addr: VirtAddr) -> ElfResult<()> {
    validate_addr(addr.as_u64())?;
    unsafe { invoke_addr(addr) };
    Ok(())
}
