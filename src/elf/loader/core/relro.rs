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
use crate::elf::types::{phdr_type, ElfHeader};
use crate::memory::addr::VirtAddr;
use crate::memory::paging::manager::api::{map_page_in_asid, translate_in_asid};
use crate::memory::paging::types::PagePermissions;

use super::parse_header::parse_program_header_at;

const PAGE: u64 = 4096;

// Drop the write bit on the PT_GNU_RELRO span (.data.rel.ro + .got) after
// relocations are applied, so a capsule cannot mutate its own GOT/relro
// after load. A write into the span then faults instead of corrupting a
// relocated pointer.
pub(in crate::elf::loader::core) fn enforce_relro(
    elf_data: &[u8],
    header: &ElfHeader,
    ph_count: usize,
    base_addr: VirtAddr,
    target_asid: u32,
) -> Result<(), ElfError> {
    let ro = PagePermissions::READ | PagePermissions::USER;
    for index in 0..ph_count {
        let ph = parse_program_header_at(elf_data, header, index)?;
        if ph.p_type != phdr_type::PT_GNU_RELRO {
            continue;
        }
        let lo = base_addr.as_u64().wrapping_add(ph.p_vaddr);
        let hi = lo.wrapping_add(ph.p_memsz);
        let mut va = lo & !(PAGE - 1);
        let end = hi.wrapping_add(PAGE - 1) & !(PAGE - 1);
        while va < end {
            if let Some(pa) = translate_in_asid(target_asid, VirtAddr::new(va)) {
                map_page_in_asid(target_asid, VirtAddr::new(va), pa, ro)
                    .map_err(|_| ElfError::RelocationFailed)?;
            }
            va = va.wrapping_add(PAGE);
        }
    }
    Ok(())
}
