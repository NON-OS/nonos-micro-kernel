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

use crate::elf::errors::ElfError;
use crate::elf::tls::{TlsInfo, DEFAULT_TLS_ALIGNMENT, TCB_SIZE};
use crate::elf::types::ProgramHeader;
use crate::memory::addr::VirtAddr;

pub(in crate::elf::loader::core) fn parse_tls_section(
    ph: &ProgramHeader,
    base_addr: VirtAddr,
) -> Result<TlsInfo, ElfError> {
    let template_size = usize::try_from(ph.p_filesz).map_err(|_| ElfError::TlsSectionError)?;
    let memory_size = usize::try_from(ph.p_memsz).map_err(|_| ElfError::TlsSectionError)?;
    let alignment = usize::try_from(ph.p_align).map_err(|_| ElfError::AlignmentError)?;
    let template_addr =
        base_addr.as_u64().checked_add(ph.p_vaddr).ok_or(ElfError::AddressOverflow)?;
    if memory_size < template_size {
        return Err(ElfError::TlsSectionError);
    }
    if ph.p_align > 1 && !ph.p_align.is_power_of_two() {
        return Err(ElfError::AlignmentError);
    }
    let effective_alignment = alignment.max(1).max(DEFAULT_TLS_ALIGNMENT);
    let alloc_size =
        memory_size.checked_add(effective_alignment - 1).ok_or(ElfError::TlsSectionError)?;
    let alloc_size = alloc_size & !(effective_alignment - 1);
    template_addr.checked_add(template_size as u64).ok_or(ElfError::AddressOverflow)?;
    alloc_size.checked_add(TCB_SIZE).ok_or(ElfError::TlsSectionError)?;
    Ok(TlsInfo::new(VirtAddr::new(template_addr), template_size, memory_size, alignment))
}
