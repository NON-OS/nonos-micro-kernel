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

use super::globals::{PAGING_MANAGER, PAGING_STATS};
use crate::arch::x86_64::idt::without_interrupts;
use crate::memory::addr::{PhysAddr, VirtAddr};
use crate::memory::paging::constants::{pages_needed, PAGE_SIZE_4K};
use crate::memory::paging::error::PagingResult;
use crate::memory::paging::types::{PagePermissions, PageSize};

pub fn map_page(
    virtual_addr: VirtAddr,
    physical_addr: PhysAddr,
    permissions: PagePermissions,
) -> PagingResult<()> {
    // Disable interrupts across the PAGING_MANAGER critical section.
    // Without this, a timer ISR firing on this CPU while the lock is
    // held would try to call switch_to_process_address_space from
    // preempt_current_process and re-enter the same spin::Mutex,
    // deadlocking the CPU.
    without_interrupts(|| {
        PAGING_MANAGER.lock().map_page(
            virtual_addr,
            physical_addr,
            permissions,
            PageSize::Size4KiB,
            &PAGING_STATS,
        )
    })
}

pub fn map_huge_page(
    virtual_addr: VirtAddr,
    physical_addr: PhysAddr,
    permissions: PagePermissions,
    size: PageSize,
) -> PagingResult<()> {
    without_interrupts(|| {
        PAGING_MANAGER.lock().map_page(
            virtual_addr,
            physical_addr,
            permissions,
            size,
            &PAGING_STATS,
        )
    })
}

pub fn unmap_page(virtual_addr: VirtAddr) -> PagingResult<PhysAddr> {
    let (phys, perms, size) =
        without_interrupts(|| PAGING_MANAGER.lock().unmap_page(virtual_addr))?;
    PAGING_STATS.record_unmapping(perms, size);
    Ok(phys)
}

// Multi-page unmap. Walks 4 KiB pages from `virtual_addr` for `size`
// bytes (rounded up). Stops at the first failure and returns it.
pub fn unmap_range(virtual_addr: VirtAddr, size: usize) -> PagingResult<()> {
    for i in 0..pages_needed(size) {
        let va = VirtAddr::new(virtual_addr.as_u64() + (i * PAGE_SIZE_4K) as u64);
        unmap_page(va)?;
    }
    Ok(())
}

pub fn map_kernel_page(virtual_addr: VirtAddr, physical_addr: PhysAddr) -> PagingResult<()> {
    let permissions = PagePermissions::READ | PagePermissions::WRITE | PagePermissions::GLOBAL;
    map_page(virtual_addr, physical_addr, permissions)
}

pub fn map_user_page(
    virtual_addr: VirtAddr,
    physical_addr: PhysAddr,
    writable: bool,
) -> PagingResult<()> {
    let mut permissions = PagePermissions::READ | PagePermissions::USER;
    if writable {
        permissions = permissions | PagePermissions::WRITE;
    }
    map_page(virtual_addr, physical_addr, permissions)
}

pub fn map_device_memory(
    virtual_addr: VirtAddr,
    physical_addr: PhysAddr,
    size: usize,
) -> PagingResult<()> {
    let permissions = PagePermissions::READ
        | PagePermissions::WRITE
        | PagePermissions::NO_CACHE
        | PagePermissions::DEVICE;
    for i in 0..pages_needed(size) {
        let va = VirtAddr::new(virtual_addr.as_u64() + (i * PAGE_SIZE_4K) as u64);
        let pa = PhysAddr::new(physical_addr.as_u64() + (i * PAGE_SIZE_4K) as u64);
        map_page(va, pa, permissions)?;
    }
    Ok(())
}

// Map an MMIO range into the currently active user address space.
// Permissions: user, read+write, uncached, no-execute. Caller is the
// hardware broker; no other path is allowed to expose physical
// memory to a capsule. The caller is responsible for ensuring the
// physical range belongs to a BAR claimed by the calling pid; this
// helper does not consult the broker tables.
pub fn map_user_mmio(
    virtual_addr: VirtAddr,
    physical_addr: PhysAddr,
    size: usize,
) -> PagingResult<()> {
    let permissions = PagePermissions::USER
        | PagePermissions::READ
        | PagePermissions::WRITE
        | PagePermissions::NO_CACHE
        | PagePermissions::DEVICE;
    let pages = pages_needed(size);
    for i in 0..pages {
        let va = VirtAddr::new(virtual_addr.as_u64() + (i * PAGE_SIZE_4K) as u64);
        let pa = PhysAddr::new(physical_addr.as_u64() + (i * PAGE_SIZE_4K) as u64);
        if let Err(e) = map_page(va, pa, permissions) {
            for j in 0..i {
                let rb = VirtAddr::new(virtual_addr.as_u64() + (j * PAGE_SIZE_4K) as u64);
                let _ = unmap_page(rb);
            }
            return Err(e);
        }
    }
    Ok(())
}

// Unmap an MMIO range previously installed by `map_user_mmio`. The
// range must lie in the active address space; broker grant
// revocation calls this from the holder pid's exit context or from
// the device-release syscall, both of which run with the correct
// CR3 active. The unmap path emits a per-asid SMP TLB shootdown.
pub fn unmap_user_mmio(virtual_addr: VirtAddr, size: usize) -> PagingResult<()> {
    for i in 0..pages_needed(size) {
        let va = VirtAddr::new(virtual_addr.as_u64() + (i * PAGE_SIZE_4K) as u64);
        let _ = unmap_page(va);
    }
    Ok(())
}

// Map a DMA buffer into the currently active user address space.
// Permissions: user, read+write, no-execute, write-back cacheable.
// On x86_64 PCI devices snoop the cache, so write-back is the
// correct cache mode for coherent DMA; uncached / write-combining
// would be wrong here. Caller is the hardware broker; no other
// path is allowed to expose physical memory to a capsule.
pub fn map_user_dma(
    virtual_addr: VirtAddr,
    physical_addr: PhysAddr,
    size: usize,
) -> PagingResult<()> {
    let permissions = PagePermissions::USER | PagePermissions::READ | PagePermissions::WRITE;
    let pages = pages_needed(size);
    for i in 0..pages {
        let va = VirtAddr::new(virtual_addr.as_u64() + (i * PAGE_SIZE_4K) as u64);
        let pa = PhysAddr::new(physical_addr.as_u64() + (i * PAGE_SIZE_4K) as u64);
        if let Err(e) = map_page(va, pa, permissions) {
            for j in 0..i {
                let rb = VirtAddr::new(virtual_addr.as_u64() + (j * PAGE_SIZE_4K) as u64);
                let _ = unmap_page(rb);
            }
            return Err(e);
        }
    }
    Ok(())
}

// Unmap a DMA range previously installed by `map_user_dma`. The
// SMP TLB shootdown happens inside `unmap_page` via the per-asid
// flush in the paging manager.
pub fn unmap_user_dma(virtual_addr: VirtAddr, size: usize) -> PagingResult<()> {
    for i in 0..pages_needed(size) {
        let va = VirtAddr::new(virtual_addr.as_u64() + (i * PAGE_SIZE_4K) as u64);
        let _ = unmap_page(va);
    }
    Ok(())
}
