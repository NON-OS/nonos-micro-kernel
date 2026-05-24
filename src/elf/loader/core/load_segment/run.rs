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

use x86_64::structures::paging::PageTableFlags;

use crate::elf::errors::ElfError;
use crate::elf::loader::image::LoadedSegment;
use crate::elf::types::ProgramHeader;
use crate::memory::addr::VirtAddr;

use super::populate_page::populate_page;
use super::pte_flags::pte_perms_from_phdr;

const PAGE: usize = 4096;
const USER_VA_MAX: u64 = 0x0000_7FFF_FFFF_FFFF;

// Map a PT_LOAD segment into `target_asid`'s address space.
//
// Honours a non-page-aligned `p_vaddr`: the bytes that belong at
// VA = base_addr + p_vaddr land at frame offset (p_vaddr & 0xFFF)
// of the first page; the data spills onto subsequent pages from
// offset 0. memsz > filesz is treated as bss — the trailing
// segment bytes stay zero (`populate_page` zeroes each frame
// before the copy).
//
// Every arithmetic step that touches lengths or VAs is checked.
// This loader chooses the first CPL=3 instruction; an unchecked
// overflow here would let the kernel iretq into something nobody
// audited.
pub(in crate::elf::loader::core) fn load_segment(
    elf_data: &[u8],
    ph: &ProgramHeader,
    base_addr: VirtAddr,
    target_asid: u32,
) -> Result<LoadedSegment, ElfError> {
    if ph.p_filesz > ph.p_memsz {
        return Err(ElfError::SegmentDataOutOfBounds);
    }
    if ph.is_writable() && ph.is_executable() {
        return Err(ElfError::WXViolation);
    }

    let file_size = ph.p_filesz as usize;
    let seg_size = ph.p_memsz as usize;
    let file_offset = ph.p_offset as usize;
    let intra = (ph.p_vaddr & 0xFFF) as usize;

    let file_end = file_offset.checked_add(file_size).ok_or(ElfError::SegmentDataOutOfBounds)?;
    if file_end > elf_data.len() {
        return Err(ElfError::SegmentDataOutOfBounds);
    }

    let total_bytes = intra.checked_add(seg_size).ok_or(ElfError::AddressOverflow)?;
    let pages = total_bytes.checked_add(PAGE - 1).ok_or(ElfError::AddressOverflow)? / PAGE;

    let span = (pages as u64).checked_mul(PAGE as u64).ok_or(ElfError::AddressOverflow)?;
    let aligned_off = ph.p_vaddr - (intra as u64);
    let seg_start = base_addr.as_u64().checked_add(ph.p_vaddr).ok_or(ElfError::AddressOverflow)?;
    let aligned_start =
        base_addr.as_u64().checked_add(aligned_off).ok_or(ElfError::AddressOverflow)?;
    let span_end = aligned_start.checked_add(span).ok_or(ElfError::AddressOverflow)?;
    if seg_start > USER_VA_MAX || span_end == 0 || span_end > USER_VA_MAX {
        return Err(ElfError::InvalidAddress);
    }

    let perms = pte_perms_from_phdr(ph);
    let file_bytes = &elf_data[file_offset..file_end];
    let seg_va = VirtAddr::new(seg_start);
    let seg_va_aligned = VirtAddr::new(aligned_start);

    for i in 0..pages {
        let page_off = (i as u64).checked_mul(PAGE as u64).ok_or(ElfError::AddressOverflow)?;
        let page_va =
            VirtAddr::new(aligned_start.checked_add(page_off).ok_or(ElfError::AddressOverflow)?);
        let dst_off = if i == 0 { intra } else { 0 };

        // Bytes consumed from the segment before this page begins.
        // The first page contributes `PAGE - intra` segment bytes;
        // every subsequent page contributes a full PAGE.
        let consumed_segment = if i == 0 {
            0usize
        } else {
            (i.checked_mul(PAGE).ok_or(ElfError::AddressOverflow)?)
                .checked_sub(intra)
                .ok_or(ElfError::AddressOverflow)?
        };
        let bytes_in_page = PAGE - dst_off;

        let src: &[u8] = if consumed_segment >= file_size {
            &[]
        } else {
            let take = file_size
                .checked_sub(consumed_segment)
                .ok_or(ElfError::AddressOverflow)?
                .min(bytes_in_page);
            let end = consumed_segment.checked_add(take).ok_or(ElfError::AddressOverflow)?;
            &file_bytes[consumed_segment..end]
        };

        populate_page(target_asid, page_va, perms, dst_off, src)?;
    }

    let _ = seg_va_aligned;

    let mut flags = PageTableFlags::PRESENT | PageTableFlags::USER_ACCESSIBLE;
    if ph.is_writable() {
        flags |= PageTableFlags::WRITABLE;
    }
    if !ph.is_executable() {
        flags |= PageTableFlags::NO_EXECUTE;
    }

    Ok(LoadedSegment { vaddr: seg_va, size: seg_size, flags, segment_type: ph.p_type })
}
