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

//! Pure validator for `MkPciConfigWrite`. The whole authority lives
//! in this function: only PCI Command bit 2 (Bus Master Enable) and
//! the MSI-X Message Control register's Function Mask + Enable bits
//! may flip. Every other config-space write — BAR programming,
//! interrupt line, IDs, status, expansion ROM, capability pointer
//! mutation, PCIe / AER — is rejected before it reaches the bus.

use crate::drivers::pci::constants::{
    CFG_COMMAND, CMD_BUS_MASTER, CMD_MEMORY_SPACE, MSIX_CTRL_ENABLE, MSIX_CTRL_FUNCTION_MASK,
};
use crate::drivers::pci::types::MsixInfo;

use super::types::{PciWriteError, PciWriteRequest, WriteAction};

const MSIX_CONTROL_WRITABLE: u16 = MSIX_CTRL_ENABLE | MSIX_CTRL_FUNCTION_MASK;

// The only PCI Command bits a driver capsule may flip: Bus Master (for DMA)
// and Memory Space (so its MMIO BAR is decoded). Firmware often leaves
// Memory Space clear on an LPSS controller it did not use, and then every
// MMIO register access silently drops, so a driver must be able to assert
// it. No other command bit (I/O space, interrupt disable, SERR, etc.) is
// writable through this path.
const COMMAND_WRITABLE: u16 = CMD_BUS_MASTER | CMD_MEMORY_SPACE;

pub fn validate(
    req: &PciWriteRequest,
    msix: Option<&MsixInfo>,
    current_register: u16,
) -> Result<WriteAction, PciWriteError> {
    if req.offset == CFG_COMMAND as u32 {
        return validate_command(req.value, current_register);
    }
    if let Some(m) = msix {
        let ctrl_offset = (m.offset as u32) + 2;
        if req.offset == ctrl_offset {
            return validate_msix_control(req.value, current_register, ctrl_offset as u16);
        }
    }
    Err(PciWriteError::OffsetNotAllowed)
}

fn validate_command(new: u16, current: u16) -> Result<WriteAction, PciWriteError> {
    let desired = if new & !COMMAND_WRITABLE == 0 { current | new } else { new };
    if (desired ^ current) & !COMMAND_WRITABLE != 0 {
        return Err(PciWriteError::BitsNotAllowed);
    }
    Ok(WriteAction::Command(desired))
}

fn validate_msix_control(
    new: u16,
    current: u16,
    offset: u16,
) -> Result<WriteAction, PciWriteError> {
    if (new ^ current) & !MSIX_CONTROL_WRITABLE != 0 {
        return Err(PciWriteError::BitsNotAllowed);
    }
    Ok(WriteAction::MsixControl { offset, value: new })
}
