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
use crate::memory::addr::{PhysAddr, VirtAddr};
use core::sync::atomic::Ordering;

use super::pcb::ProcessControlBlock;
use super::types::{align_up, overlaps, Vma};

impl ProcessControlBlock {
    pub fn mmap(
        &self,
        hint: Option<VirtAddr>,
        length: usize,
        flags: u64,
    ) -> Result<VirtAddr, &'static str> {
        if length == 0 {
            return Err("EINVAL");
        }
        let pages = (length + 4095) / 4096;
        let map_flags =
            flags::PRESENT | flags::USER | (flags & (flags::WRITABLE | flags::NO_EXECUTE));

        let mut mem = self.memory.lock();

        let va = match hint {
            Some(h) if (h.as_u64() & 0xFFF) == 0 && !overlaps(&mem.vmas, h, length) => h,
            _ => {
                let mut candidate = align_up(mem.next_va, 0x1000);
                let upper_bound: u64 = 0x0000_FFFF_FFFF_F000;
                loop {
                    if candidate > upper_bound {
                        return Err("ENOMEM");
                    }
                    let cand = VirtAddr::new(candidate);
                    if !overlaps(&mem.vmas, cand, length) {
                        break cand;
                    }
                    candidate = align_up(candidate + length as u64, 0x1000);
                }
            }
        };

        let mut allocated_pages: usize = 0;

        let result = (|| -> Result<(), &'static str> {
            for i in 0..pages {
                let page_va = VirtAddr::new(va.as_u64() + (i as u64) * 4096);
                let phys = allocate_physical_page().ok_or("ENOMEM")?;
                map_page_to_phys(page_va, phys, map_flags).map_err(|_| "EIO")?;
                allocated_pages += 1;
                // SAFETY: Page was just mapped with valid physical memory backing.
                unsafe {
                    core::ptr::write_bytes(page_va.as_u64() as *mut u8, 0, 4096);
                }
            }
            Ok(())
        })();

        if result.is_err() {
            for i in 0..allocated_pages {
                let page_va = VirtAddr::new(va.as_u64() + (i as u64) * 4096);
                let _ = unmap_range(page_va, 4096);
            }
            return result.map(|_| va);
        }

        mem.vmas.push(Vma {
            start: va,
            end: VirtAddr::new(va.as_u64() + length as u64),
            flags: map_flags,
        });
        mem.resident_pages.fetch_add(pages as u64, Ordering::Relaxed);
        mem.next_va = align_up(va.as_u64() + length as u64, 0x1000);
        Ok(va)
    }

    pub fn munmap(&self, addr: VirtAddr, length: usize) -> Result<(), &'static str> {
        if length == 0 || (addr.as_u64() & 0xFFF) != 0 {
            return Err("EINVAL");
        }
        let end = addr.as_u64().checked_add(length as u64).ok_or("EINVAL")?;

        let mut mem = self.memory.lock();
        let mut i = 0usize;
        while i < mem.vmas.len() {
            let v = &mem.vmas[i];
            let vs = v.start.as_u64();
            let ve = v.end.as_u64();

            if end <= vs || addr.as_u64() >= ve {
                i += 1;
                continue;
            }

            let unmap_start = addr.as_u64().max(vs);
            let unmap_end = end.min(ve);
            let unmap_len = (unmap_end - unmap_start) as usize;

            unmap_range(VirtAddr::new(unmap_start), unmap_len).map_err(|_| "EIO")?;
            mem.resident_pages.fetch_sub(((unmap_len + 4095) / 4096) as u64, Ordering::Relaxed);

            if unmap_start == vs && unmap_end == ve {
                mem.vmas.swap_remove(i);
                continue;
            } else if unmap_start == vs {
                mem.vmas[i].start = VirtAddr::new(unmap_end);
                i += 1;
            } else if unmap_end == ve {
                mem.vmas[i].end = VirtAddr::new(unmap_start);
                i += 1;
            } else {
                let right = Vma { start: VirtAddr::new(unmap_end), end: v.end, flags: v.flags };
                mem.vmas[i].end = VirtAddr::new(unmap_start);
                mem.vmas.push(right);
                i += 1;
            }
        }

        Ok(())
    }
}

fn allocate_physical_page() -> Option<PhysAddr> {
    crate::memory::phys::alloc(crate::memory::phys::AllocFlags::empty())
        .map(|f| crate::memory::addr::PhysAddr::new(f.0))
}

fn map_page_to_phys(page_va: VirtAddr, phys: PhysAddr, _flags: u64) -> Result<(), ()> {
    use crate::memory::paging::types::PagePermissions;
    let perms = PagePermissions::READ | PagePermissions::WRITE;
    crate::memory::paging::manager::map_page(page_va, phys, perms).map_err(|_| ())
}

fn unmap_range(addr: VirtAddr, len: usize) -> Result<(), ()> {
    crate::memory::paging::manager::unmap_range(addr, len).map_err(|_| ())
}
