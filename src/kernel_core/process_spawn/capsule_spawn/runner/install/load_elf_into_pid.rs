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

use super::super::super::spec::SpawnError;
use crate::elf::loader::load_elf_entry_into;
use crate::memory::paging::manager::lookup_asid_for_process;

pub(super) fn load_elf_into_pid(
    elf: &'static [u8],
    pid: u32,
    debug_tag: &'static [u8],
) -> Result<u64, SpawnError> {
    let asid = lookup_asid_for_process(pid).ok_or(SpawnError::AddressSpace)?;
    load_elf_entry_into(elf, asid).map_err(|err| {
        crate::sys::serial::println(debug_tag);
        crate::sys::serial::println(err.as_str().as_bytes());
        SpawnError::ElfLoad
    })
}
