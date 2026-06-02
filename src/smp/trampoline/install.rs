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

use super::asm::{
    nonos_ap_trampoline_cpu_id, nonos_ap_trampoline_end, nonos_ap_trampoline_entry,
    nonos_ap_trampoline_pml4, nonos_ap_trampoline_ready, nonos_ap_trampoline_stack,
    nonos_ap_trampoline_start,
};
use super::per_ap::PerApBootContext;
use crate::memory::addr::PhysAddr;
use crate::memory::unified::phys_to_virt_checked;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrampolineError {
    AddressTooHigh,
    AddressMisaligned,
    PayloadTooLarge,
    DirectmapMiss,
    Pml4Above4G,
}

const TRAMPOLINE_MAX_BYTES: usize = 4096;

pub fn install_trampoline_at(addr: PhysAddr) -> Result<(), TrampolineError> {
    let phys = addr.as_u64();
    if phys >= 0x10_0000 {
        return Err(TrampolineError::AddressTooHigh);
    }
    if phys & 0xFFF != 0 {
        return Err(TrampolineError::AddressMisaligned);
    }

    let (src, len) = trampoline_image();
    if len > TRAMPOLINE_MAX_BYTES {
        return Err(TrampolineError::PayloadTooLarge);
    }

    let dst_virt = phys_to_virt_checked(addr).ok_or(TrampolineError::DirectmapMiss)?;
    // SAFETY: dst is the directmap alias of the verified low-1M trampoline frame;
    // src is a kernel rodata blob of length `len`; both are non-overlapping.
    unsafe {
        let dst = dst_virt.as_u64() as *mut u8;
        core::ptr::copy_nonoverlapping(src, dst, len);
    }

    Ok(())
}

// The trampoline fields are emitted after `.balign 8` in tables.S and the
// image is installed at a page-aligned address, so every field offset is
// naturally aligned for its width; the volatile pointer casts below are sound.
#[allow(clippy::cast_ptr_alignment)]
pub fn write_per_ap_context(addr: PhysAddr, ctx: &PerApBootContext) -> Result<(), TrampolineError> {
    if ctx.pml4_phys > u32::MAX as u64 {
        return Err(TrampolineError::Pml4Above4G);
    }

    let base_virt = phys_to_virt_checked(addr).ok_or(TrampolineError::DirectmapMiss)?;
    let base = base_virt.as_u64() as *mut u8;

    let off_pml4 = field_offset(&raw const nonos_ap_trampoline_pml4);
    let off_stack = field_offset(&raw const nonos_ap_trampoline_stack);
    let off_entry = field_offset(&raw const nonos_ap_trampoline_entry);
    let off_cpuid = field_offset(&raw const nonos_ap_trampoline_cpu_id);
    let off_ready = field_offset(&raw const nonos_ap_trampoline_ready);

    // SAFETY: All four fields live inside the trampoline image whose
    // length is bounded by `trampoline_image`. The destination is the
    // freshly installed trampoline page at `addr`.
    unsafe {
        core::ptr::write_volatile(base.add(off_pml4) as *mut u64, ctx.pml4_phys);
        core::ptr::write_volatile(base.add(off_stack) as *mut u64, ctx.stack_top);
        core::ptr::write_volatile(base.add(off_entry) as *mut u64, ctx.entry_ptr);
        core::ptr::write_volatile(base.add(off_cpuid) as *mut u32, ctx.cpu_id);
        core::ptr::write_volatile(base.add(off_ready) as *mut u32, 0);
    }

    Ok(())
}

// `ready` sits at a 4-aligned offset (see tables.S) in the page-aligned
// trampoline image, so the volatile u32 read is correctly aligned.
#[allow(clippy::cast_ptr_alignment)]
pub fn ap_signaled_ready(addr: PhysAddr) -> bool {
    let Some(base_virt) = phys_to_virt_checked(addr) else { return false };
    let off_ready = field_offset(&raw const nonos_ap_trampoline_ready);
    // SAFETY: trampoline page is mapped read-write through the directmap.
    unsafe {
        let p = base_virt.as_u64() as *const u8;
        core::ptr::read_volatile(p.add(off_ready) as *const u32) != 0
    }
}

fn trampoline_image() -> (*const u8, usize) {
    let start = &raw const nonos_ap_trampoline_start as *const u8;
    let end = &raw const nonos_ap_trampoline_end as *const u8;
    // SAFETY: both symbols are defined in the same rodata blob and end > start.
    let len = unsafe { end.offset_from(start) } as usize;
    (start, len)
}

fn field_offset(field: *const u8) -> usize {
    let start = &raw const nonos_ap_trampoline_start as usize;
    field as usize - start
}
