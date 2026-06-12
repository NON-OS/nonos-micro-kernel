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

// Top-level orchestration for ET_EXEC kernel loading. Two placement
// modes:
//
//   1. Low-half (legacy): the ELF declares a fixed phys VA inside
//      the low 4 GiB. UEFI is asked to allocate at exactly that
//      address. phys == virt for the rest of the boot.
//
//   2. Upper-half (NØNOS canonical): the ELF declares an upper-half
//      virt window. UEFI is asked for any contiguous phys range;
//      the loader copies each PT_LOAD at phys + (p_vaddr - min_vaddr)
//      and records (phys, virt, size, flags) per segment so the
//      paging stage can install phys -> virt mappings with the right
//      permissions before the CR3 swap.

extern crate alloc;

use alloc::format;

use crate::crypto::sig::CapsuleMetadata;
use crate::loader::errors::{LoaderError, LoaderResult};
use crate::loader::image::KernelImage;
use crate::loader::types::memory;
use crate::log::logger::{log_error, log_info};

use crate::loader::core::alloc::free_all;
use crate::loader::core::constants::{MAX_ALLOCS, PAGE_SIZE};
use crate::loader::core::types::ValidationResult;

use super::allocate_image::allocate_image;
use super::entry_in_range::entry_in_range;
use super::load_segments::load_segments;

pub fn load_exec_kernel(
    bs: &uefi::table::boot::BootServices,
    payload: &[u8],
    v: &ValidationResult,
) -> LoaderResult<KernelImage> {
    let virt_min = v.min_addr;
    let total_bytes = (v.max_addr - v.min_addr) as usize;
    let pages_needed = (total_bytes + PAGE_SIZE - 1) / PAGE_SIZE;
    let upper_half = memory::is_upper_half(virt_min);

    let mut allocations: [(u64, usize); MAX_ALLOCS] = [(0, 0); MAX_ALLOCS];
    let mut alloc_count: usize = 0;

    let phys_base =
        allocate_image(bs, virt_min, pages_needed, upper_half, &mut allocations, &mut alloc_count)?;

    let (segments, segment_count) = load_segments(payload, v, phys_base, virt_min);

    let entry = v.elf.header.e_entry as usize;
    if !entry_in_range(entry, upper_half, virt_min, phys_base, total_bytes) {
        free_all(bs, &allocations, alloc_count);
        log_error("loader", "ELF entry not in loaded segments");
        return Err(LoaderError::EntryNotInRange);
    }

    let virt_base = if upper_half { virt_min } else { 0 };

    let image = KernelImage {
        address: phys_base as usize,
        size: total_bytes,
        entry_point: entry,
        virt_base,
        segments,
        segment_count,
        metadata: CapsuleMetadata {
            offset_sig: 0,
            len_sig: 0,
            offset_payload: 0,
            len_payload: payload.len(),
            signer_keyid: None,
            payload_hash: [0u8; 32],
            header_version: 1,
            header_timestamp: 0,
        },
        allocations: [(0, 0); memory::MAX_ALLOCATIONS],
        alloc_count: 0,
    };

    log_info(
        "loader",
        &format!(
            "Kernel loaded: phys=0x{:x} virt=0x{:x} size=0x{:x} entry=0x{:x}",
            image.address, image.virt_base, image.size, image.entry_point,
        ),
    );
    Ok(image)
}
