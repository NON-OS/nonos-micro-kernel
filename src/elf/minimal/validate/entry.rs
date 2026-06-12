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

use crate::elf::errors::{ElfError, ElfResult};

use super::parse::parse_header;

pub fn entry_from_bytes(bytes: &[u8]) -> ElfResult<u64> {
    let header = parse_header(bytes)?;
    if !header.is_valid_magic() {
        return Err(ElfError::InvalidMagic);
    }
    if header.e_entry == 0 {
        return Err(ElfError::Other("Invalid entry point"));
    }
    Ok(header.e_entry)
}
