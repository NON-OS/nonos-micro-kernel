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

/*
 * ET_DYN kernel loading.
 * Position-independent executables with relocation support.
 */

extern crate alloc;

use alloc::format;
use uefi::prelude::Status;
use uefi::table::boot::{AllocateType, MemoryType};

use crate::crypto::sig::CapsuleMetadata;
use crate::loader::errors::{LoaderError, LoaderResult};
use crate::loader::image::{KernelImage, KernelSegmentLayout, MAX_KERNEL_SEGMENTS};
use crate::loader::types::memory;
use crate::log::logger::{log_error, log_info};

use super::alloc::record_alloc;
use super::constants::{MAX_ALLOCS, PAGE_SIZE};
use super::types::ValidationResult;

pub fn load_dyn_kernel(
    bs: &uefi::table::boot::BootServices,
    payload: &[u8],
    v: &ValidationResult,
) -> LoaderResult<KernelImage> {
    let base = v.min_addr;
    let total_bytes = (v.max_addr - v.min_addr) as usize;
    let pages_needed = (total_bytes + PAGE_SIZE - 1) / PAGE_SIZE;

    let mut allocations: [(u64, usize); MAX_ALLOCS] = [(0, 0); MAX_ALLOCS];
    let mut alloc_count: usize = 0;

    let alloc_addr =
        match bs.allocate_pages(AllocateType::AnyPages, MemoryType::LOADER_DATA, pages_needed) {
            Ok(addr) => {
                record_alloc(&mut allocations, &mut alloc_count, addr, pages_needed)?;
                log_info(
                    "loader",
                    &format!("Allocated {} pages at 0x{:x} (ET_DYN)", pages_needed, addr),
                );
                addr
            }
            Err(e) => {
                log_error("loader", &format!("ET_DYN allocation failed: {:?}", e.status()));
                return Err(LoaderError::AllocationFailed {
                    addr: 0,
                    pages: pages_needed,
                    status: e.status(),
                });
            }
        };

    let base_phys = alloc_addr as u64;
    load_segments_relocated(payload, v, base, base_phys);

    let load_bias = (base_phys as i64) - (base as i64);
    apply_relocations(v, base_phys, load_bias, payload)?;

    let entry_virt = v.elf.header.e_entry as u64;
    let entry_offset = entry_virt.saturating_sub(base);
    let entry_phys = base_phys.checked_add(entry_offset).ok_or(LoaderError::UefiError {
        desc: "entry overflow",
        status: Status::OUT_OF_RESOURCES,
    })? as usize;

    let image = KernelImage {
        address: base_phys as usize,
        size: pages_needed * PAGE_SIZE,
        entry_point: entry_phys,
        virt_base: 0,
        segments: [KernelSegmentLayout::default(); MAX_KERNEL_SEGMENTS],
        segment_count: 0,
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
            "ET_DYN loaded: 0x{:x} size=0x{:x} entry=0x{:x}",
            image.address, image.size, image.entry_point
        ),
    );
    Ok(image)
}

fn load_segments_relocated(payload: &[u8], v: &ValidationResult, base: u64, base_phys: u64) {
    for i in 0..v.load_count {
        let seg = &v.loads[i];
        let rel = (seg.target as u64).wrapping_sub(base);
        let dst = (base_phys + rel) as usize;

        if seg.p_filesz > 0 {
            unsafe {
                core::ptr::copy_nonoverlapping(
                    payload.as_ptr().add(seg.p_offset),
                    dst as *mut u8,
                    seg.p_filesz,
                );
            }
        }

        if seg.p_memsz > seg.p_filesz {
            unsafe {
                core::ptr::write_bytes(
                    (dst + seg.p_filesz) as *mut u8,
                    0,
                    seg.p_memsz - seg.p_filesz,
                );
            }
        }
    }
}

fn apply_relocations(
    v: &ValidationResult,
    base_phys: u64,
    load_bias: i64,
    payload: &[u8],
) -> LoaderResult<()> {
    match crate::loader::reloc::process_elf_relocations(&v.elf, base_phys, load_bias, payload) {
        Ok(reloc_count) => {
            if reloc_count > 0 {
                log_info("loader", &format!("Applied {} relocations", reloc_count));
            }
            Ok(())
        }
        Err(e) => {
            log_error("loader", &format!("SECURITY: relocation failed: {}", e));
            Err(LoaderError::MalformedElf("relocation failed"))
        }
    }
}
