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

//! Assert PCI Memory Space so the controller's MMIO BAR is decoded.
//!
//! Firmware frequently hands off an LPSS I2C controller it never used with
//! the Memory Space command bit clear. Every MMIO register access then
//! silently drops, so the LPSS reset and the DesignWare bring-up write into
//! the void and the touchpad stays dead. Assert Memory Space (and Bus
//! Master, harmless for this PIO driver) once, right after the claim, before
//! any BAR access.

use nonos_libc::{
    mk_pci_config_write, MK_PCI_CFG_COMMAND, MK_PCI_CMD_BUS_MASTER, MK_PCI_CMD_MEMORY_SPACE,
};

pub fn enable(device_id: u64, claim_epoch: u64) -> Result<(), &'static str> {
    let bits = MK_PCI_CMD_MEMORY_SPACE | MK_PCI_CMD_BUS_MASTER;
    if mk_pci_config_write(device_id, claim_epoch, MK_PCI_CFG_COMMAND, bits) < 0 {
        return Err("i2c-pci: could not enable memory space");
    }
    Ok(())
}
