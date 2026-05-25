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
use crate::memory::addr::VirtAddr;

use super::plan::SegmentPlan;

const PAGE: usize = 4096;

pub(super) struct PageCopy<'a> {
    pub page_va: VirtAddr,
    pub dst_off: usize,
    pub src: &'a [u8],
}

pub(super) fn page<'a>(
    file_bytes: &'a [u8],
    plan: &SegmentPlan,
    page_index: usize,
) -> Result<PageCopy<'a>, ElfError> {
    let page_off = (page_index as u64).checked_mul(PAGE as u64).ok_or(ElfError::AddressOverflow)?;
    let page_va = VirtAddr::new(plan.aligned_start.checked_add(page_off).ok_or(ElfError::AddressOverflow)?);
    let dst_off = if page_index == 0 { plan.intra } else { 0 };
    let consumed_segment = if page_index == 0 {
        0
    } else {
        page_index.checked_mul(PAGE).ok_or(ElfError::AddressOverflow)?
            .checked_sub(plan.intra).ok_or(ElfError::AddressOverflow)?
    };
    let bytes_in_page = PAGE - dst_off;
    let src = if consumed_segment >= plan.file_size {
        &[]
    } else {
        let take = plan.file_size.checked_sub(consumed_segment).ok_or(ElfError::AddressOverflow)?.min(bytes_in_page);
        let end = consumed_segment.checked_add(take).ok_or(ElfError::AddressOverflow)?;
        &file_bytes[consumed_segment..end]
    };
    Ok(PageCopy { page_va, dst_off, src })
}
