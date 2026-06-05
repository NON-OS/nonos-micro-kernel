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
use crate::memory::layout::DIRECTMAP_BASE;
use crate::memory::paging::manager::api::translate_in_asid;

use super::parse_header::parse_program_header_at;

const DT_RELA: u64 = 7;
const DT_RELASZ: u64 = 8;
const R_X86_64_RELATIVE: u64 = 8;
const DYN_ENTRY: usize = 16;
const RELA_ENTRY: usize = 24;

fn rd_u64(bytes: &[u8], off: usize) -> u64 {
    u64::from_le_bytes([
        bytes[off],
        bytes[off + 1],
        bytes[off + 2],
        bytes[off + 3],
        bytes[off + 4],
        bytes[off + 5],
        bytes[off + 6],
        bytes[off + 7],
    ])
}

pub(in crate::elf::loader::core) fn apply_relative_relocations(
    elf_data: &[u8],
    header: &ElfHeader,
    ph_count: usize,
    base_addr: VirtAddr,
    target_asid: u32,
) -> Result<(), ElfError> {
    let mut rela_vaddr: Option<u64> = None;
    let mut rela_size = 0u64;
    for index in 0..ph_count {
        let ph = parse_program_header_at(elf_data, header, index)?;
        if ph.p_type != phdr_type::PT_DYNAMIC {
            continue;
        }
        let mut off = ph.p_offset as usize;
        let end = off.saturating_add(ph.p_filesz as usize).min(elf_data.len());
        while off + DYN_ENTRY <= end {
            match rd_u64(elf_data, off) {
                DT_RELA => rela_vaddr = Some(rd_u64(elf_data, off + 8)),
                DT_RELASZ => rela_size = rd_u64(elf_data, off + 8),
                0 => break,
                _ => {}
            }
            off += DYN_ENTRY;
        }
    }
    let Some(rela_vaddr) = rela_vaddr else { return Ok(()) };
    let file_off = vaddr_to_file_offset(elf_data, header, ph_count, rela_vaddr)?;
    let mut at = 0usize;
    while at + RELA_ENTRY <= rela_size as usize && file_off + at + RELA_ENTRY <= elf_data.len() {
        let entry = file_off + at;
        let r_offset = rd_u64(elf_data, entry);
        let r_info = rd_u64(elf_data, entry + 8);
        let r_addend = rd_u64(elf_data, entry + 16);
        at += RELA_ENTRY;
        if r_info & 0xffff_ffff != R_X86_64_RELATIVE {
            continue;
        }
        let va = VirtAddr::new(base_addr.as_u64().wrapping_add(r_offset));
        let value = base_addr.as_u64().wrapping_add(r_addend);
        let pa = translate_in_asid(target_asid, va).ok_or(ElfError::DynamicSectionError)?;
        unsafe {
            core::ptr::write((DIRECTMAP_BASE + pa.as_u64()) as *mut u64, value);
        }
    }
    Ok(())
}

fn vaddr_to_file_offset(
    elf_data: &[u8],
    header: &ElfHeader,
    ph_count: usize,
    vaddr: u64,
) -> Result<usize, ElfError> {
    for index in 0..ph_count {
        let ph = parse_program_header_at(elf_data, header, index)?;
        let end = ph.p_vaddr.wrapping_add(ph.p_filesz);
        if ph.p_type == phdr_type::PT_LOAD && vaddr >= ph.p_vaddr && vaddr < end {
            return Ok((ph.p_offset + (vaddr - ph.p_vaddr)) as usize);
        }
    }
    Err(ElfError::DynamicSectionError)
}
