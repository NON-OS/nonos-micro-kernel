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
use crate::elf::types::ProgramHeader;
use crate::memory::addr::VirtAddr;

use super::validate::input_fields;

const PAGE: usize = 4096;
const USER_VA_MAX: u64 = 0x0000_7FFF_FFFF_FFFF;

pub(super) struct SegmentPlan {
    pub file_offset: usize,
    pub file_size: usize,
    pub file_end: usize,
    pub seg_size: usize,
    pub intra: usize,
    pub pages: usize,
    pub seg_va: VirtAddr,
    pub aligned_start: u64,
}

pub(super) fn build(
    elf_data: &[u8],
    header: &ProgramHeader,
    base_addr: VirtAddr,
) -> Result<SegmentPlan, ElfError> {
    let (file_offset, file_size, file_end, seg_size, intra) = input_fields(elf_data.len(), header)?;
    let total_bytes = intra.checked_add(seg_size).ok_or(ElfError::AddressOverflow)?;
    let pages = total_bytes.checked_add(PAGE - 1).ok_or(ElfError::AddressOverflow)? / PAGE;
    let span = (pages as u64).checked_mul(PAGE as u64).ok_or(ElfError::AddressOverflow)?;
    let aligned_off = header.p_vaddr.checked_sub(intra as u64).ok_or(ElfError::AddressOverflow)?;
    let seg_start = base_addr.as_u64().checked_add(header.p_vaddr).ok_or(ElfError::AddressOverflow)?;
    let aligned_start = base_addr.as_u64().checked_add(aligned_off).ok_or(ElfError::AddressOverflow)?;
    let span_end = aligned_start.checked_add(span).ok_or(ElfError::AddressOverflow)?;
    if seg_start > USER_VA_MAX || span_end == 0 || span_end > USER_VA_MAX {
        return Err(ElfError::InvalidAddress);
    }
    Ok(SegmentPlan {
        file_offset,
        file_size,
        file_end,
        seg_size,
        intra,
        pages,
        seg_va: VirtAddr::new(seg_start),
        aligned_start,
    })
}
