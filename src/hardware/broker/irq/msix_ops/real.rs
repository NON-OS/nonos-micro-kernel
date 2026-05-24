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

// x86-only: PCI MSI-X capability access + MSI-X table layout.
#![cfg(target_arch = "x86_64")]

use crate::drivers::pci::config::ConfigSpace;
use crate::drivers::pci::msi::{
    configure_msix, disable_msix, enable_msix, mask_all_msix, mask_msix_vector, unmask_all_msix,
    unmask_msix_vector,
};
use crate::drivers::pci::types::{MsixInfo, PciAddress, PciBar};

use super::super::types::IrqBindError;
use super::mmio_zero::zero_table_entry;
use super::ops::MsixOps;

struct RealMsixOps;

impl MsixOps for RealMsixOps {
    fn program_run(
        &self,
        address: &PciAddress,
        msix: &MsixInfo,
        bars: &[PciBar; 6],
        base_vector: u8,
        count: usize,
        dest_apic_id: u8,
    ) -> Result<(), IrqBindError> {
        let cfg = ConfigSpace::new(*address);
        mask_all_msix(&cfg, msix).map_err(|_| IrqBindError::MsixProgramFailed)?;
        enable_msix(&cfg, msix).map_err(|_| IrqBindError::MsixProgramFailed)?;
        for i in 0..count {
            let vector = i as u16;
            configure_msix(&cfg, msix, bars, vector, base_vector + i as u8, dest_apic_id)
                .map_err(|_| IrqBindError::MsixProgramFailed)?;
            unmask_msix_vector(msix, bars, vector).map_err(|_| IrqBindError::MsixProgramFailed)?;
        }
        unmask_all_msix(&cfg, msix).map_err(|_| IrqBindError::MsixProgramFailed)
    }

    fn teardown_vector(
        &self,
        _address: &PciAddress,
        msix: &MsixInfo,
        bars: &[PciBar; 6],
        device_vector: u16,
    ) {
        let _ = mask_msix_vector(msix, bars, device_vector);
        zero_table_entry(msix, bars, device_vector);
    }

    fn disable_for_device(&self, address: &PciAddress, msix: &MsixInfo) {
        let cfg = ConfigSpace::new(*address);
        let _ = mask_all_msix(&cfg, msix);
        let _ = disable_msix(&cfg, msix);
    }
}

static REAL_OPS: RealMsixOps = RealMsixOps;

#[cfg(not(test))]
pub(in crate::hardware::broker::irq) fn current_ops() -> &'static dyn MsixOps {
    &REAL_OPS
}
