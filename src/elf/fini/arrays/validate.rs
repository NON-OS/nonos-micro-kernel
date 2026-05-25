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

use super::info::FiniArrayInfo;

pub(super) fn validate_addr(addr: u64) -> ElfResult<()> {
    if addr == 0 { return Err(ElfError::InvalidAddress); }
    Ok(())
}

pub(super) fn validate_array(info: &FiniArrayInfo) -> ElfResult<()> {
    if info.is_empty() { return Ok(()); }
    validate_addr(info.addr.as_u64())?;
    if !info.is_entry_aligned() { return Err(ElfError::AlignmentError); }
    info.end_addr().ok_or(ElfError::AddressOverflow).map(|_| ())
}
