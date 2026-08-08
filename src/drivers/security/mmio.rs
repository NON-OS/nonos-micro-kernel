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

use super::constants::*;
use super::error::DriverError;
use crate::memory::addr::VirtAddr;

pub fn validate_mmio_region(base: usize, size: usize) -> Result<(), DriverError> {
    if !super::mmio_range::range_ok(base, size) {
        return Err(DriverError::InvalidMmioRegion);
    }

    if base % PAGE_SIZE != 0 {
        return Err(DriverError::InvalidMmioRegion);
    }

    let is_low_mmio = base >= LOW_MMIO_START && base < LOW_MMIO_END;
    let is_platform_mmio = base >= PLATFORM_MMIO_START && base < PLATFORM_MMIO_END;
    let is_high_mmio = base >= HIGH_MMIO_START;

    if !is_low_mmio && !is_platform_mmio && !is_high_mmio {
        return Err(DriverError::InvalidMmioRegion);
    }

    Ok(())
}

pub fn safe_mmio_read32(addr: VirtAddr) -> Result<u32, DriverError> {
    let addr_usize = addr.as_u64() as usize;

    if addr_usize % 4 != 0 {
        return Err(DriverError::MmioAccessDenied);
    }

    validate_mmio_region(addr_usize & !0xFFF, PAGE_SIZE)?;

    // SAFETY: Address has been validated as within MMIO region
    let val = unsafe { core::ptr::read_volatile(addr_usize as *const u32) };

    Ok(val)
}

pub fn safe_mmio_write32(addr: VirtAddr, val: u32) -> Result<(), DriverError> {
    let addr_usize = addr.as_u64() as usize;

    if addr_usize % 4 != 0 {
        return Err(DriverError::MmioAccessDenied);
    }

    validate_mmio_region(addr_usize & !0xFFF, PAGE_SIZE)?;

    // SAFETY: Address has been validated as within MMIO region
    unsafe {
        core::ptr::write_volatile(addr_usize as *mut u32, val);
    }

    Ok(())
}

pub fn safe_mmio_read64(addr: VirtAddr) -> Result<u64, DriverError> {
    let addr_usize = addr.as_u64() as usize;

    if addr_usize % 8 != 0 {
        return Err(DriverError::MmioAccessDenied);
    }

    validate_mmio_region(addr_usize & !0xFFF, PAGE_SIZE)?;

    // SAFETY: Address has been validated as within MMIO region
    let val = unsafe { core::ptr::read_volatile(addr_usize as *const u64) };

    Ok(val)
}

pub fn safe_mmio_write64(addr: VirtAddr, val: u64) -> Result<(), DriverError> {
    let addr_usize = addr.as_u64() as usize;

    if addr_usize % 8 != 0 {
        return Err(DriverError::MmioAccessDenied);
    }

    validate_mmio_region(addr_usize & !0xFFF, PAGE_SIZE)?;

    // SAFETY: Address has been validated as within MMIO region
    unsafe {
        core::ptr::write_volatile(addr_usize as *mut u64, val);
    }

    Ok(())
}
