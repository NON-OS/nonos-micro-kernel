// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use crate::loader::errors::{LoaderError, LoaderResult};
use crate::loader::types::{memory, ph_flags, ph_type, Elf64Header, Elf64Phdr, LoadedSegment};

use super::context::ValidationContext;

pub fn validate_program_header(
    phdr: &Elf64Phdr,
    file_size: usize,
    ctx: &mut ValidationContext,
) -> LoaderResult<Option<LoadedSegment>> {
    if phdr.p_type == ph_type::PT_DYNAMIC {
        ctx.has_dynamic = true;
    }

    if phdr.p_type != ph_type::PT_LOAD {
        return Ok(None);
    }

    if ctx.segment_count >= memory::MAX_LOAD_SEGMENTS {
        return Err(LoaderError::TooManySegments);
    }

    if phdr.p_memsz < phdr.p_filesz {
        return Err(LoaderError::InvalidSegmentSize);
    }

    let file_end = phdr.p_offset.checked_add(phdr.p_filesz).ok_or(LoaderError::IntegerOverflow)?;

    if file_end as usize > file_size {
        return Err(LoaderError::SegmentOutOfBounds);
    }

    let target = phdr.p_vaddr;
    if target == 0 && !ctx.is_pie {
        return Err(LoaderError::UnsupportedElf("segment has no address"));
    }

    // Upper-half kernel images are staged at a bootloader-chosen phys
    // frame and live at their canonical virt address. Low-half range
    // checks (MIN_LOAD_ADDRESS / MAX_LOAD_ADDRESS) are placement guards
    // and do not apply to upper-half virt targets.
    let upper_half_target = memory::is_upper_half(target);

    if target != 0 && !upper_half_target && target < memory::MIN_LOAD_ADDRESS {
        return Err(LoaderError::AddressOutOfRange);
    }

    let seg_end = target.checked_add(phdr.p_memsz).ok_or(LoaderError::IntegerOverflow)?;

    if !upper_half_target && seg_end > memory::MAX_LOAD_ADDRESS {
        return Err(LoaderError::AddressOutOfRange);
    }

    let is_writable = (phdr.p_flags & ph_flags::PF_W) != 0;
    let is_executable = (phdr.p_flags & ph_flags::PF_X) != 0;
    if is_writable && is_executable {
        ctx.wx_segments += 1;
    }

    if target != 0 {
        ctx.min_addr = ctx.min_addr.min(target);
        ctx.max_addr = ctx.max_addr.max(seg_end);
    }
    ctx.segment_count += 1;

    Ok(Some(LoadedSegment::from_phdr(phdr)))
}

pub fn validate_program_headers(
    data: &[u8],
    header: &Elf64Header,
    ctx: &mut ValidationContext,
) -> LoaderResult<[Option<LoadedSegment>; memory::MAX_LOAD_SEGMENTS]> {
    let mut segments = [None; memory::MAX_LOAD_SEGMENTS];
    let mut load_idx = 0;

    let ph_offset = header.e_phoff as usize;
    let ph_size = core::mem::size_of::<Elf64Phdr>();

    for i in 0..header.e_phnum as usize {
        let offset = ph_offset + i * ph_size;

        if offset + ph_size > data.len() {
            return Err(LoaderError::SegmentOutOfBounds);
        }

        // SAFETY: We've validated bounds and alignment
        let phdr = unsafe { &*(data.as_ptr().add(offset) as *const Elf64Phdr) };

        if let Some(segment) = validate_program_header(phdr, data.len(), ctx)? {
            if load_idx < memory::MAX_LOAD_SEGMENTS {
                segments[load_idx] = Some(segment);
                load_idx += 1;
            }
        }
    }

    if ctx.segment_count == 0 {
        return Err(LoaderError::NoLoadableSegments);
    }

    if ctx.min_addr != u64::MAX {
        ctx.total_size =
            ctx.max_addr.checked_sub(ctx.min_addr).ok_or(LoaderError::IntegerOverflow)? as usize;
    }

    if ctx.total_size > memory::MAX_KERNEL_SIZE {
        return Err(LoaderError::KernelTooLarge);
    }

    if ctx.total_size == 0 {
        return Err(LoaderError::MalformedElf("zero size kernel"));
    }

    Ok(segments)
}
