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

extern crate alloc;
use super::super::image::ElfImage;
use super::section::ParsedSection;
use crate::elf::aslr::AslrManager;
use crate::elf::errors::ElfError;
use crate::elf::types::*;
use crate::memory::addr::VirtAddr;
use alloc::{collections::BTreeMap, string::String, vec::Vec};

pub(super) const DEFAULT_STATIC_BASE: u64 = 0x400000;
pub(super) const DEFAULT_PIE_BASE: u64 = 0x400000;

pub struct ElfLoader {
    pub(super) aslr_manager: AslrManager,
    pub(super) loaded_libraries: BTreeMap<String, ElfImage>,
    pub(super) symbol_cache: BTreeMap<String, VirtAddr>,
}

impl ElfLoader {
    pub fn new() -> Self {
        ElfLoader {
            aslr_manager: AslrManager::new(),
            loaded_libraries: BTreeMap::new(),
            symbol_cache: BTreeMap::new(),
        }
    }

    pub fn load_executable_into(
        &mut self,
        elf_data: &[u8],
        target_asid: u32,
    ) -> Result<ElfImage, ElfError> {
        let header = super::parse_header::parse_elf_header(elf_data)?;
        super::parse_header::validate_elf(&header)?;
        let program_headers = super::parse_header::parse_program_headers(elf_data, &header)?;
        let base_addr = if header.e_type == elf_type::ET_DYN {
            VirtAddr::new(self.aslr_manager.randomize_base(DEFAULT_PIE_BASE))
        } else {
            VirtAddr::new(DEFAULT_STATIC_BASE)
        };
        let (mut loaded_segments, mut dynamic_info, mut tls_info, mut interpreter) =
            (Vec::new(), None, None, None);
        for ph in &program_headers {
            match ph.p_type {
                phdr_type::PT_LOAD => {
                    loaded_segments.push(super::load_segment::load_segment(
                        elf_data,
                        ph,
                        base_addr,
                        target_asid,
                    )?);
                }
                phdr_type::PT_DYNAMIC => {
                    dynamic_info =
                        Some(super::parse_dynamic::parse_dynamic_section(elf_data, ph, base_addr)?);
                }
                phdr_type::PT_TLS => {
                    tls_info = Some(super::parse_dynamic::parse_tls_section(ph, base_addr)?);
                }
                phdr_type::PT_INTERP => {
                    interpreter = Some(super::parse_dynamic::parse_interpreter(elf_data, ph)?);
                }
                _ => {}
            }
        }
        let entry_point = if header.e_type == elf_type::ET_DYN {
            base_addr + header.e_entry
        } else {
            VirtAddr::new(header.e_entry)
        };
        let total_size = loaded_segments.iter().map(|seg| seg.size).sum();
        let image = ElfImage {
            base_addr,
            entry_point,
            size: total_size,
            memory_size: total_size,
            segments: loaded_segments,
            dynamic_info,
            dynlink_info: None,
            tls_info,
            interpreter,
        };
        Ok(image)
    }

    pub fn load_entry_into(
        &mut self,
        elf_data: &[u8],
        target_asid: u32,
    ) -> Result<VirtAddr, ElfError> {
        let header = super::parse_header::parse_elf_header(elf_data)?;
        super::parse_header::validate_elf(&header)?;
        let (_, _, ph_count) = super::parse_header::program_header_bounds(elf_data, &header)?;
        let base_addr = if header.e_type == elf_type::ET_DYN {
            VirtAddr::new(self.aslr_manager.randomize_base(DEFAULT_PIE_BASE))
        } else {
            VirtAddr::new(DEFAULT_STATIC_BASE)
        };
        for i in 0..ph_count {
            let ph = super::parse_header::parse_program_header_at(elf_data, &header, i)?;
            if ph.p_type == phdr_type::PT_LOAD {
                let _ = super::load_segment::load_segment(elf_data, &ph, base_addr, target_asid)?;
            }
        }
        let entry_point = if header.e_type == elf_type::ET_DYN {
            base_addr + header.e_entry
        } else {
            VirtAddr::new(header.e_entry)
        };
        Ok(entry_point)
    }

    pub fn load_executable(&mut self, elf_data: &[u8]) -> Result<ElfImage, ElfError> {
        let active =
            crate::memory::paging::manager::active_asid().ok_or(ElfError::NotInitialized)?;
        self.load_executable_into(elf_data, active)
    }

    pub fn load_library(&mut self, elf_data: &[u8]) -> Result<ElfImage, ElfError> {
        self.load_executable(elf_data)
    }
    pub fn library_count(&self) -> usize {
        self.loaded_libraries.len()
    }
    pub fn symbol_count(&self) -> usize {
        self.symbol_cache.len()
    }
    pub fn clear_symbol_cache(&mut self) {
        self.symbol_cache.clear();
    }
    pub fn parse_section_headers(&self, elf_data: &[u8]) -> Result<Vec<ParsedSection>, ElfError> {
        super::parse_sections::parse_section_headers(elf_data)
    }
    pub fn find_section_by_name<'a>(
        sections: &'a [ParsedSection],
        name: &str,
    ) -> Option<&'a ParsedSection> {
        sections.iter().find(|s| s.name == name)
    }
    pub fn get_symbol_table<'a>(sections: &'a [ParsedSection]) -> Option<&'a ParsedSection> {
        sections.iter().find(|s| s.is_symtab())
    }
    pub fn get_dynsym<'a>(sections: &'a [ParsedSection]) -> Option<&'a ParsedSection> {
        sections.iter().find(|s| s.section_type == 11)
    }
}

impl Default for ElfLoader {
    fn default() -> Self {
        Self::new()
    }
}
