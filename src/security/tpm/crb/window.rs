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

use core::sync::atomic::{AtomicU64, Ordering};

use super::regs::{TPM_MMIO_BASE, TPM_MMIO_SIZE};
use crate::memory::addr::PhysAddr;
use crate::memory::mmio::map_device_memory;
use crate::security::tpm::error::TpmError;

/// Virtual address of the mapped register window, or zero before bring-up.
/// The kernel differs from the bootloader here: there is no identity map, so
/// the window has to be mapped before a single register can be read.
static WINDOW: AtomicU64 = AtomicU64::new(0);

/// Map the register window once. Idempotent, so a second caller reuses the
/// first mapping rather than creating an alias of the same device memory.
pub(super) fn init_window() -> Result<u64, TpmError> {
    let existing = WINDOW.load(Ordering::Acquire);
    if existing != 0 {
        return Ok(existing);
    }
    let va = map_device_memory(PhysAddr::new(TPM_MMIO_BASE), TPM_MMIO_SIZE)
        .map_err(|_| TpmError::NotPresent)?
        .as_u64();
    match WINDOW.compare_exchange(0, va, Ordering::AcqRel, Ordering::Acquire) {
        Ok(_) => Ok(va),
        Err(winner) => Ok(winner),
    }
}

pub(super) fn window() -> Result<u64, TpmError> {
    match WINDOW.load(Ordering::Acquire) {
        0 => Err(TpmError::NotPresent),
        va => Ok(va),
    }
}

pub(super) fn read32(offset: u32) -> Result<u32, TpmError> {
    let base = window()?;
    debug_assert!((offset as usize) + 4 <= TPM_MMIO_SIZE);
    // SAFETY: eK@nonos.systems - `base` is the live device mapping created by
    // `init_window`, the offset is inside the window it covers, and TPM
    // registers are uncached device memory that tolerates a 32-bit read.
    Ok(unsafe { core::ptr::read_volatile((base as usize + offset as usize) as *const u32) })
}

/// # Safety
/// Writing a TPM control register starts or cancels a command. The caller owns
/// the sequencing the part expects around it.
pub(super) unsafe fn write32(offset: u32, value: u32) -> Result<(), TpmError> {
    let base = window()?;
    debug_assert!((offset as usize) + 4 <= TPM_MMIO_SIZE);
    // SAFETY: eK@nonos.systems - as `read32` for the mapping and bounds; the
    // caller promised the sequencing this register requires.
    unsafe {
        core::ptr::write_volatile((base as usize + offset as usize) as *mut u32, value);
    }
    Ok(())
}
