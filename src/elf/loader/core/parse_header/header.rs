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
use crate::elf::types::ElfHeader;
use core::ptr;

pub(crate) fn parse_elf_header(elf_data: &[u8]) -> Result<ElfHeader, ElfError> {
    if elf_data.len() < ElfHeader::SIZE {
        return Err(ElfError::FileTooSmall);
    }
    unsafe { Ok(ptr::read_unaligned(elf_data.as_ptr() as *const ElfHeader)) }
}
