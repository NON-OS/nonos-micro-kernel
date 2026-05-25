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

use crate::elf::types::{elf_class, elf_data, elf_machine, elf_type, ELF_MAGIC};

pub(super) fn make_valid_elf_header() -> [u8; 64] {
    let mut header = [0u8; 64];
    header[0..4].copy_from_slice(&ELF_MAGIC);
    header[4] = elf_class::ELFCLASS64;
    header[5] = elf_data::ELFDATA2LSB;
    header[6] = 1;
    header[16] = (elf_type::ET_EXEC & 0xFF) as u8;
    header[17] = ((elf_type::ET_EXEC >> 8) & 0xFF) as u8;
    header[18] = (elf_machine::EM_X86_64 & 0xFF) as u8;
    header[19] = ((elf_machine::EM_X86_64 >> 8) & 0xFF) as u8;
    header[24..32].copy_from_slice(&0x401000u64.to_le_bytes());
    header[32..40].copy_from_slice(&64u64.to_le_bytes());
    header[56..58].copy_from_slice(&3u16.to_le_bytes());
    header
}
