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

//! Pure validation routines used by the `MkIrqBind` MSI-X path.
//! Everything in this module is a function over plain inputs — no
//! globals, no MMIO, no allocation. The bind path runs the
//! validators after looking up the kernel-side state, which keeps
//! the test surface small enough to verify in a host-side crate.
//!
//! Errors are returned in a fixed priority order so a capsule
//! gets a deterministic explanation for a malformed request:
//!
//!   1. `UnsupportedFlags` — flag bits the kernel does not know.
//!   2. `BadVectorCount`   — count is zero, larger than the broker
//!                           pool, or larger than the device's MSI-X
//!                           table.
//!   3. `NoDeviceHandle`   — broker has no PCI side-table entry for
//!                           this `device_id` (e.g. platform device).
//!   4. `NoMsixCap`        — device does not advertise an MSI-X cap.
//!   5. `BadMsixBar`       — table or PBA BAR is out of range, not
//!                           memory-mapped, or not present.
//!   6. `NotDeviceIrq`     — MSI-X mode requires `irq_source == 0`.
//!   7. `AlreadyBound`     — pid already holds an MSI-X grant for
//!                           this device; MSI-X bind is all-or-nothing
//!                           per device per pid.

use super::types::{IrqBindError, IrqBindRequest, BIND_MSIX, FLAGS_KNOWN};

/// Snapshot of the kernel-side per-device state the validator needs.
/// Builders (production and tests) construct one of these from the
/// real `pci_index` entry; the validator itself only inspects the
/// fields and never goes back to the kernel.
#[derive(Clone, Copy, Debug)]
pub(super) struct MsixHandleView {
    pub(super) msix_present: bool,
    pub(super) msix_table_size: u16,
    pub(super) table_bar_in_range: bool,
    pub(super) table_bar_is_mmio: bool,
    pub(super) pba_bar_in_range: bool,
    pub(super) pba_bar_is_mmio: bool,
}

impl MsixHandleView {
    pub(super) const fn no_msix() -> Self {
        Self {
            msix_present: false,
            msix_table_size: 0,
            table_bar_in_range: false,
            table_bar_is_mmio: false,
            pba_bar_in_range: false,
            pba_bar_is_mmio: false,
        }
    }
}

/// Validate the MSI-X branch of `MkIrqBind`. Returns `Ok` only if
/// every check passes; the caller may then proceed to allocate
/// broker slots and program the device. `pool_capacity` is the
/// broker vector pool's total size, passed in so the validator
/// stays free of any global lookup. `handle` is `None` when the
/// broker has no PCI side-table entry for this device id.
pub(super) fn validate_msix_request(
    req: &IrqBindRequest,
    pool_capacity: usize,
    handle: Option<&MsixHandleView>,
    has_existing_msix_grant: bool,
) -> Result<(), IrqBindError> {
    if req.flags & !FLAGS_KNOWN != 0 {
        return Err(IrqBindError::UnsupportedFlags);
    }
    if req.flags & BIND_MSIX == 0 {
        // Caller routed an INTx request through the MSI-X validator;
        // returning UnsupportedFlags here keeps the contract one-sided
        // (validator handles only the MSI-X branch).
        return Err(IrqBindError::UnsupportedFlags);
    }

    let n = req.vector_count as usize;
    if n == 0 || n > pool_capacity {
        return Err(IrqBindError::BadVectorCount);
    }

    let handle = handle.ok_or(IrqBindError::NoDeviceHandle)?;
    if !handle.msix_present {
        return Err(IrqBindError::NoMsixCap);
    }
    if (n as u16) > handle.msix_table_size {
        return Err(IrqBindError::BadVectorCount);
    }
    if !handle.table_bar_in_range
        || !handle.table_bar_is_mmio
        || !handle.pba_bar_in_range
        || !handle.pba_bar_is_mmio
    {
        return Err(IrqBindError::BadMsixBar);
    }
    if req.irq_source != 0 {
        return Err(IrqBindError::NotDeviceIrq);
    }
    if has_existing_msix_grant {
        return Err(IrqBindError::AlreadyBound);
    }

    Ok(())
}

/// Validate the INTx branch. The MSI-X bind path must not call
/// this; the regular `bind_intx` flow does, and the validator's
/// presence here lets the host test crate cover the INTx-side
/// rejections (`NotIntx`, `NotDeviceIrq`, `BadVectorCount`) as
/// pure functions too.
pub(super) fn validate_intx_request(
    req: &IrqBindRequest,
    irq_pin: u8,
    irq_line: u8,
) -> Result<(), IrqBindError> {
    if req.flags & !FLAGS_KNOWN != 0 {
        return Err(IrqBindError::UnsupportedFlags);
    }
    if req.flags & BIND_MSIX != 0 {
        return Err(IrqBindError::UnsupportedFlags);
    }
    if req.vector_count != 0 {
        return Err(IrqBindError::BadVectorCount);
    }
    if irq_pin == 0 || irq_line == 0xFF {
        return Err(IrqBindError::NotIntx);
    }
    if req.irq_source != irq_line as u32 {
        return Err(IrqBindError::NotDeviceIrq);
    }
    Ok(())
}
