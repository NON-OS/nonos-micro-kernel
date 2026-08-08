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

//! Exec-time mapping seam: stack mapping with guard, and recording of
//! ELF segments onto the PCB so the release path can find them.

use alloc::sync::Arc;

use crate::elf::loader::LoadedSegment;
use crate::memory::addr::VirtAddr;
use crate::process::core::types::Vma;
use crate::process::core::ProcessControlBlock;

pub fn map_user_stack(
    pcb: &Arc<ProcessControlBlock>,
    top: VirtAddr,
    size: usize,
) -> Result<(), &'static str> {
    use crate::arch::paging::descriptor::flags;
    let bottom = top.as_u64().saturating_sub(size as u64);
    let pages = (size + 4095) / 4096;
    let perms = crate::memory::paging::types::PagePermissions::user_rw();
    for i in 0..pages {
        let va = VirtAddr::new(bottom + (i as u64) * 4096);
        let pa =
            crate::memory::frame_alloc::allocate_frame().ok_or("failed to allocate stack frame")?;
        crate::memory::paging::map_page(va, pa, perms).map_err(|_| "failed to map stack page")?;
    }
    // The page at `bottom - 4096` is left unmapped as a guard.
    // A stack overflow that touches it faults through the trap policy
    // as SIGSEGV.
    let mut mem = pcb.memory.lock();
    mem.vmas.push(Vma {
        start: VirtAddr::new(bottom),
        end: top,
        flags: flags::PRESENT | flags::WRITABLE | flags::USER | flags::NO_EXECUTE,
    });
    Ok(())
}

pub fn record_segments(pcb: &Arc<ProcessControlBlock>, segments: &[LoadedSegment]) {
    let mut mem = pcb.memory.lock();
    for seg in segments {
        let start = seg.vaddr;
        let end = VirtAddr::new(seg.vaddr.as_u64() + seg.size as u64);
        mem.vmas.push(Vma { start, end, flags: seg.flags });
    }
}
