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

use crate::elf::errors::ElfError;
use crate::elf::loader::image::LoadedSegment;
use crate::elf::types::ProgramHeader;
use crate::memory::addr::VirtAddr;

use super::plan;
use super::populate_page::populate_page;
use super::pte_flags::pte_perms_from_phdr;
use super::result;
use super::source;

pub(in crate::elf::loader::core) fn load_segment(
    elf_data: &[u8],
    header: &ProgramHeader,
    base_addr: VirtAddr,
    target_asid: u32,
) -> Result<LoadedSegment, ElfError> {
    let plan = plan::build(elf_data, header, base_addr)?;
    let perms = pte_perms_from_phdr(header);
    let file_bytes = &elf_data[plan.file_offset..plan.file_end];
    crate::sys::serial::print(b"[LOAD-DBG] seg pages=");
    crate::sys::serial::print_dec(plan.pages as u64);
    crate::sys::serial::print(b"\n");
    for page_index in 0..plan.pages {
        if page_index % 512 == 0 {
            crate::sys::serial::print(b"[LOAD-DBG] p");
            crate::sys::serial::print_dec(page_index as u64);
            crate::sys::serial::print(b"\n");
        }
        let copy = source::page(file_bytes, &plan, page_index)?;
        populate_page(target_asid, copy.page_va, perms, copy.dst_off, copy.src)?;
    }
    crate::sys::serial::print(b"[LOAD-DBG] seg ok\n");
    Ok(result::loaded_segment(&plan, header))
}
