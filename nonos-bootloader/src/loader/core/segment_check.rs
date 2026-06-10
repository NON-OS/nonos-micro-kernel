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

use goblin::elf::ProgramHeader;

use crate::loader::errors::{LoaderError, LoaderResult};
use crate::loader::types::memory;
use crate::log::logger::{log_error, log_info, log_warn};

use super::constants::{elf_flags, PAGE_SIZE};
use super::types::ValidatedSegment;

pub fn validate_single_segment(
    ph: &ProgramHeader,
    payload: &[u8],
    is_exec: bool,
) -> LoaderResult<ValidatedSegment> {
    let p_offset = ph.p_offset as usize;
    let p_filesz = ph.p_filesz as usize;
    let p_memsz = ph.p_memsz as usize;
    let p_flags = ph.p_flags;

    if p_memsz < p_filesz {
        log_error("loader", "SECURITY: Segment has p_memsz < p_filesz (malformed ELF)");
        return Err(LoaderError::InvalidSegmentSize);
    }

    let file_end = p_offset.checked_add(p_filesz).ok_or_else(|| {
        log_error("loader", "SECURITY: Integer overflow in segment offset+size");
        LoaderError::IntegerOverflow
    })?;

    if file_end > payload.len() {
        log_error("loader", "ELF program header indicates file data outside payload bounds.");
        return Err(LoaderError::SegmentOutOfBounds);
    }

    let is_writable = (p_flags & elf_flags::PF_W) != 0;
    let is_executable = (p_flags & elf_flags::PF_X) != 0;
    if is_writable && is_executable {
        log_warn("loader", "WARNING: Segment has W+X permissions (W^X violation)");
    }

    let target = if ph.p_paddr != 0 { ph.p_paddr } else { ph.p_vaddr } as u64;

    if target == 0 && is_exec {
        log_error("loader", "ET_EXEC PT_LOAD has no placement address.");
        return Err(LoaderError::UnsupportedElf("no placement address"));
    }

    // The MIN_LOAD_ADDRESS guard exists to keep low-half ET_EXEC images
    // out of UEFI/ACPI/firmware reservations. Upper-half kernels are
    // staged at a bootloader-chosen physical frame and only ever live
    // at their canonical virt; the low-half check does not apply.
    if is_exec && !memory::is_upper_half(target) && target < memory::MIN_LOAD_ADDRESS {
        log_error("loader", "SECURITY: Load address too low (below 64MB)");
        return Err(LoaderError::AddressOutOfRange);
    }

    Ok(ValidatedSegment {
        p_offset,
        p_filesz,
        p_memsz,
        target,
        p_align: ph.p_align as usize,
        p_flags,
    })
}

pub fn compute_segment_bounds(seg: &ValidatedSegment, is_exec: bool) -> LoaderResult<(u64, u64)> {
    let base_page = seg.target & !((PAGE_SIZE as u64) - 1);
    let offset_into_page = (seg.target - base_page) as usize;
    let seg_start = base_page + (offset_into_page as u64);

    let seg_end = seg_start.checked_add(seg.p_memsz as u64).ok_or_else(|| {
        log_error("loader", "SECURITY: Integer overflow computing segment end");
        LoaderError::IntegerOverflow
    })?;

    // MAX_LOAD_ADDRESS guards against low-half placements that would
    // collide with the firmware-reserved high region. An upper-half
    // virt target is not a placement at all, so the check does not
    // apply.
    if is_exec && !memory::is_upper_half(seg.target) && seg_end > memory::MAX_LOAD_ADDRESS {
        log_error("loader", "SECURITY: Segment extends beyond maximum load address");
        return Err(LoaderError::AddressOutOfRange);
    }

    Ok((seg_start, seg_end))
}

pub fn finalize_bounds(min_addr: Option<u64>, max_addr: Option<u64>) -> LoaderResult<(u64, u64)> {
    let base = min_addr.ok_or_else(|| {
        log_error("loader", "Internal error: min_addr not set after segment processing");
        LoaderError::MalformedElf("min_addr computation failed")
    })?;
    let end = max_addr.ok_or_else(|| {
        log_error("loader", "Internal error: max_addr not set after segment processing");
        LoaderError::MalformedElf("max_addr computation failed")
    })?;
    Ok((base, end))
}

pub fn validate_kernel_size(base: u64, end: u64) -> LoaderResult<()> {
    let total_bytes = end.checked_sub(base).ok_or_else(|| {
        log_error("loader", "SECURITY: Size calculation underflow");
        LoaderError::IntegerOverflow
    })? as usize;

    if total_bytes > memory::MAX_KERNEL_SIZE {
        log_error("loader", "SECURITY: Kernel size exceeds maximum allowed (512 MiB)");
        return Err(LoaderError::KernelTooLarge);
    }

    if total_bytes == 0 {
        log_error("loader", "SECURITY: Kernel has zero size");
        return Err(LoaderError::MalformedElf("zero size kernel"));
    }

    log_info("loader", "Kernel size validated");
    Ok(())
}
