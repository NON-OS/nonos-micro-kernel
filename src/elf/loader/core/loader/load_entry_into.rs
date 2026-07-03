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

use super::{base_addr, state::ElfLoader};
use crate::elf::errors::ElfError;
use crate::elf::types::{phdr_type, ProgramHeader};
use crate::memory::addr::VirtAddr;

impl ElfLoader {
    pub fn load_entry_into(
        &mut self,
        elf_data: &[u8],
        target_asid: u32,
    ) -> Result<VirtAddr, ElfError> {
        let header = super::super::parse_header::parse_elf_header(elf_data)?;
        super::super::parse_header::validate_elf(&header)?;
        let (_, _, ph_count) =
            super::super::parse_header::program_header_bounds(elf_data, &header)?;
        let base_addr = base_addr::load_base(&header, &mut self.aslr_manager);
        for index in 0..ph_count {
            let header_at =
                super::super::parse_header::parse_program_header_at(elf_data, &header, index)?;
            map_load_segment(elf_data, &header_at, base_addr, target_asid)?;
        }
        super::super::relocate::apply_relative_relocations(
            elf_data,
            &header,
            ph_count,
            base_addr,
            target_asid,
        )?;
        super::super::relro::enforce_relro(elf_data, &header, ph_count, base_addr, target_asid)?;
        Ok(base_addr::entry_point(&header, base_addr))
    }
}

fn map_load_segment(
    elf_data: &[u8],
    header: &ProgramHeader,
    base_addr: VirtAddr,
    target_asid: u32,
) -> Result<(), ElfError> {
    if header.p_type == phdr_type::PT_LOAD {
        super::super::load_segment::load_segment(elf_data, header, base_addr, target_asid)?;
    }
    Ok(())
}
