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
use core::sync::atomic::Ordering;

use super::error::{ApicError, ApicResult};
use super::state::MMIO_BASE;
use crate::memory::layout::PAGE_SIZE;

#[inline(always)]
pub fn mmio_base() -> VirtAddr {
    VirtAddr::new((MMIO_BASE.load(Ordering::Acquire) as u64) & !(PAGE_SIZE as u64 - 1))
}

#[inline(always)]
pub fn mmio_r32(offset: u32) -> u32 {
    unsafe { core::ptr::read_volatile((mmio_base().as_u64() + offset as u64) as *const u32) }
}

#[inline(always)]
pub fn mmio_w32(offset: u32, val: u32) {
    unsafe { core::ptr::write_volatile((mmio_base().as_u64() + offset as u64) as *mut u32, val) }
}

pub unsafe fn map_apic_mmio(pa: PhysAddr) -> ApicResult<VirtAddr> {
    crate::memory::mmio::map_device_memory(pa, PAGE_SIZE).map_err(|_| ApicError::MmioMapFailed)
}
