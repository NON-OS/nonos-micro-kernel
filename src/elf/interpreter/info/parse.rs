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
use crate::elf::types::{phdr_type, ProgramHeader};

use super::{constants::MAX_INTERP_PATH_LEN, state::InterpreterInfo};

impl InterpreterInfo {
    pub fn from_elf(elf_data: &[u8], ph: &ProgramHeader) -> Result<Self, ElfError> {
        if ph.p_type != phdr_type::PT_INTERP {
            return Err(ElfError::InterpreterNotFound);
        }
        let file_offset =
            usize::try_from(ph.p_offset).map_err(|_| ElfError::InterpreterNotFound)?;
        let size = usize::try_from(ph.p_filesz).map_err(|_| ElfError::InterpreterNotFound)?;
        let end = file_offset.checked_add(size).ok_or(ElfError::InterpreterNotFound)?;
        if size == 0 || size > MAX_INTERP_PATH_LEN {
            return Err(ElfError::InterpreterNotFound);
        }
        if end > elf_data.len() {
            return Err(ElfError::InterpreterNotFound);
        }
        let path_bytes = &elf_data[file_offset..end];
        if path_bytes.last() != Some(&0) {
            return Err(ElfError::Other("PT_INTERP missing NUL terminator"));
        }
        let null_pos = path_bytes.iter().position(|&byte| byte == 0).unwrap_or(path_bytes.len());
        let path = core::str::from_utf8(&path_bytes[..null_pos])
            .map_err(|_| ElfError::InterpreterInvalidUtf8)?;
        if path.is_empty() {
            return Err(ElfError::InterpreterNotFound);
        }
        Ok(Self { path: path.into() })
    }
}
