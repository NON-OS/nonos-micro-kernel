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
use crate::elf::types::ElfHeader;
use crate::elf::types::elf_type;

use super::{checks::validate_elf_detailed, parse::parse_header};

fn parse_valid_header(bytes: &[u8]) -> ElfResult<ElfHeader> { validate_elf_detailed(bytes)?; parse_header(bytes) }

pub fn get_elf_type(bytes: &[u8]) -> ElfResult<u16> { Ok(parse_valid_header(bytes)?.e_type) }
pub fn get_elf_machine(bytes: &[u8]) -> ElfResult<u16> { Ok(parse_valid_header(bytes)?.e_machine) }
pub fn is_pie(bytes: &[u8]) -> ElfResult<bool> { Ok(get_elf_type(bytes)? == elf_type::ET_DYN) }
pub fn get_phoff(bytes: &[u8]) -> ElfResult<u64> { Ok(parse_valid_header(bytes)?.e_phoff) }
pub fn get_shoff(bytes: &[u8]) -> ElfResult<u64> { Ok(parse_valid_header(bytes)?.e_shoff) }
pub fn get_phnum(bytes: &[u8]) -> ElfResult<u16> { Ok(parse_valid_header(bytes)?.e_phnum) }
pub fn get_shnum(bytes: &[u8]) -> ElfResult<u16> { Ok(parse_valid_header(bytes)?.e_shnum) }
