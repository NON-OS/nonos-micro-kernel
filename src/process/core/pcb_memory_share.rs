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

use crate::arch::paging::descriptor::flags;
use crate::memory::addr::VirtAddr;
use core::sync::atomic::Ordering;

use super::pcb::ProcessControlBlock;
use super::types::{align_up, overlaps, Vma};

impl ProcessControlBlock {
    // Reserve a free user-half VA window and record it as a VMA. PTE
    // installation is the caller's responsibility; this is used by the
    // surface registry to land a foreign frame list in this AS.
    pub fn reserve_vma(&self, length: usize) -> Result<VirtAddr, &'static str> {
        if length == 0 {
            return Err("EINVAL");
        }
        let pages = (length + 4095) / 4096;
        let map_flags = flags::PRESENT | flags::USER | flags::WRITABLE;
        let mut mem = self.memory.lock();
        let upper_bound: u64 = 0x0000_FFFF_FFFF_F000;
        let mut candidate = align_up(mem.next_va, 0x1000);
        let va = loop {
            if candidate > upper_bound {
                return Err("ENOMEM");
            }
            let cand = VirtAddr::new(candidate);
            if !overlaps(&mem.vmas, cand, length) {
                break cand;
            }
            candidate = align_up(candidate + length as u64, 0x1000);
        };
        mem.vmas.push(Vma {
            start: va,
            end: VirtAddr::new(va.as_u64() + length as u64),
            flags: map_flags,
        });
        mem.resident_pages.fetch_add(pages as u64, Ordering::Relaxed);
        mem.next_va = align_up(va.as_u64() + length as u64, 0x1000);
        Ok(va)
    }
}
