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

use crate::elf::errors::ElfError;
use crate::elf::loader::{DynamicInfo, LoadedSegment};
use crate::elf::tls::TlsInfo;
use crate::elf::types::{phdr_type, ProgramHeader};
use crate::memory::addr::VirtAddr;
use alloc::{string::String, vec::Vec};

pub(super) struct LoadedParts {
    pub segments: Vec<LoadedSegment>,
    pub dynamic_info: Option<DynamicInfo>,
    pub tls_info: Option<TlsInfo>,
    pub interpreter: Option<String>,
}

pub(super) fn load(
    elf_data: &[u8],
    program_headers: &[ProgramHeader],
    base_addr: VirtAddr,
    target_asid: u32,
) -> Result<LoadedParts, ElfError> {
    let mut parts = LoadedParts { segments: Vec::new(), dynamic_info: None, tls_info: None, interpreter: None };
    for ph in program_headers {
        match ph.p_type {
            phdr_type::PT_LOAD => {
                parts.segments.push(super::super::load_segment::load_segment(
                    elf_data,
                    ph,
                    base_addr,
                    target_asid,
                )?);
            }
            phdr_type::PT_DYNAMIC => {
                let info = super::super::parse_dynamic::parse_dynamic_section(elf_data, ph, program_headers, base_addr)?;
                set_once(&mut parts.dynamic_info, info, ElfError::DynamicSectionError)?;
            }
            phdr_type::PT_TLS => {
                let info = super::super::parse_dynamic::parse_tls_section(ph, base_addr)?;
                set_once(&mut parts.tls_info, info, ElfError::TlsSectionError)?;
            }
            phdr_type::PT_INTERP => {
                let path = super::super::parse_dynamic::parse_interpreter(elf_data, ph)?;
                set_once(&mut parts.interpreter, path, ElfError::InvalidState)?;
            }
            _ => {}
        }
    }
    Ok(parts)
}

fn set_once<T>(slot: &mut Option<T>, value: T, error: ElfError) -> Result<(), ElfError> {
    if slot.is_some() { return Err(error); }
    *slot = Some(value);
    Ok(())
}
