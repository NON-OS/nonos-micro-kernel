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

use crate::memory::addr::{PhysAddr, VirtAddr};

use super::constants::*;
use super::error::{IoApicError, IoApicResult};
use crate::memory::layout::PAGE_SIZE;

pub(crate) unsafe fn map_mmio(pa: PhysAddr) -> IoApicResult<VirtAddr> {
    crate::memory::mmio::map_device_memory(pa, PAGE_SIZE).map_err(|_| IoApicError::MmioMapFailed)
}

#[inline(always)]
pub(crate) fn reg_write(base: VirtAddr, index: u32, val: u32) {
    unsafe {
        let sel = (base.as_u64() + IOREGSEL) as *mut u32;
        let win = (base.as_u64() + IOWIN) as *mut u32;
        core::ptr::write_volatile(sel, index);
        core::ptr::write_volatile(win, val);
    }
}

#[inline(always)]
pub(crate) fn reg_read(base: VirtAddr, index: u32) -> u32 {
    unsafe {
        let sel = (base.as_u64() + IOREGSEL) as *mut u32;
        let win = (base.as_u64() + IOWIN) as *const u32;
        core::ptr::write_volatile(sel, index);
        core::ptr::read_volatile(win)
    }
}

pub(crate) unsafe fn redtbl_write(base: VirtAddr, i: u32, low: u32, high: u32) {
    reg_write(base, IOREDTBL0 + (i * 2) + 1, high);
    reg_write(base, IOREDTBL0 + (i * 2), low);
}

pub(crate) unsafe fn redtbl_read(base: VirtAddr, i: u32) -> (u32, u32) {
    let high = reg_read(base, IOREDTBL0 + (i * 2) + 1);
    let low = reg_read(base, IOREDTBL0 + (i * 2));
    (low, high)
}
