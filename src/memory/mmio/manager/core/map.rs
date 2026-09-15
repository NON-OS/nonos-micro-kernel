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

use super::super::super::constants::{
    align_up, VM_FLAG_CACHE_DISABLE, VM_FLAG_NX, VM_FLAG_USER, VM_FLAG_WRITABLE,
    VM_FLAG_WRITE_COMBINE,
};
use super::super::super::error::{MmioError, MmioResult};
use super::super::super::stats::MMIO_STATS;
use super::super::super::types::{MmioFlags, MmioRegion};
use super::types::MmioManager;
use crate::memory::addr::{PhysAddr, VirtAddr};
use crate::memory::layout;

impl MmioManager {
    pub fn map_region(
        &mut self,
        pa: PhysAddr,
        size: usize,
        flags: MmioFlags,
    ) -> MmioResult<VirtAddr> {
        if size == 0 {
            return Err(MmioError::InvalidSize);
        }
        if pa.as_u64() % layout::PAGE_SIZE as u64 != 0 {
            return Err(MmioError::NotPageAligned);
        }
        let va = self.allocate_virtual_range(size)?;
        let aligned_size = align_up(size, layout::PAGE_SIZE);
        let page_count = aligned_size / layout::PAGE_SIZE;
        let vm_flags = flags.to_vm_flags();
        for i in 0..page_count {
            let offset = i.checked_mul(layout::PAGE_SIZE).ok_or(MmioError::Overflow)? as u64;
            let page_va =
                VirtAddr::new(va.as_u64().checked_add(offset).ok_or(MmioError::Overflow)?);
            let page_pa =
                PhysAddr::new(pa.as_u64().checked_add(offset).ok_or(MmioError::Overflow)?);
            self.map_page(page_va, page_pa, vm_flags)?;
        }
        let region = MmioRegion::new(va, pa, aligned_size, flags, MMIO_STATS.next_id());
        self.regions.insert(va, region);
        MMIO_STATS.record_mapping(aligned_size);
        Ok(va)
    }

    pub fn unmap_region(&mut self, va: VirtAddr) -> MmioResult<()> {
        let region = self.regions.remove(&va).ok_or(MmioError::RegionNotFound)?;
        for i in 0..(region.size / layout::PAGE_SIZE) {
            let offset = match i.checked_mul(layout::PAGE_SIZE) {
                Some(o) => o as u64,
                None => return Err(MmioError::Overflow),
            };
            let page_va = match va.as_u64().checked_add(offset) {
                Some(a) => VirtAddr::new(a),
                None => return Err(MmioError::Overflow),
            };
            self.unmap_page(page_va)?;
        }
        MMIO_STATS.record_unmapping(region.size);
        Ok(())
    }

    fn map_page(&self, va: VirtAddr, pa: PhysAddr, vm_flags: u32) -> MmioResult<()> {
        use crate::memory::paging::manager;
        use crate::memory::paging::types::PagePermissions;
        let mut perms = PagePermissions::READ;
        if (vm_flags & VM_FLAG_WRITABLE) != 0 {
            perms = perms | PagePermissions::WRITE;
        }
        if (vm_flags & VM_FLAG_USER) != 0 {
            perms = perms | PagePermissions::USER;
        }
        if (vm_flags & VM_FLAG_NX) == 0 {
            perms = perms | PagePermissions::EXECUTE;
        }
        /*
         * Cache attributes, which this used to drop. Every caller asks for
         * uncached and every mapping came out write-back, so a register read
         * could be served from a cache line and a command write could sit in
         * one. QEMU dispatches MMIO by address and ignores the attribute, so
         * the emulator answers the same either way and nothing here ever
         * showed it.
         *
         * Write-combining is not expressible yet and is deliberately not
         * forced to uncached. Nothing programs IA32_PAT or the MTRRs, so under
         * the reset PAT there is no write-combining entry to select. The
         * framebuffer is the only caller that asks, it is the one mapping
         * where uncached costs a full-screen blit an order of magnitude, and
         * write-back is what it has always had. Leaving it there keeps that
         * unchanged; programming the PAT is what makes the request honest.
         */
        if (vm_flags & VM_FLAG_WRITE_COMBINE) != 0 {
            return manager::map_page(va, pa, perms).map_err(|_| MmioError::MappingFailed);
        }
        if (vm_flags & VM_FLAG_CACHE_DISABLE) != 0 {
            perms = perms | PagePermissions::NO_CACHE | PagePermissions::DEVICE;
        }
        manager::map_page(va, pa, perms).map_err(|_| MmioError::MappingFailed)
    }

    fn unmap_page(&self, va: VirtAddr) -> MmioResult<()> {
        crate::memory::paging::manager::unmap_page(va)
            .map(|_| ())
            .map_err(|_| MmioError::UnmapFailed)
    }
}
