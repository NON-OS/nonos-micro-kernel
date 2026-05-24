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

//! `MkPciConfigWrite`. Cap requirement: `Driver`. The kernel only
//! accepts a write into the PCI Command register's Bus Master
//! Enable bit, or into the device's MSI-X Message Control register's
//! Function Mask and Enable bits. Every other offset and every
//! other bit pattern is rejected; the constants below are the only
//! values a capsule can usefully pass.

use crate::syscall::{call_raw, N_MK_PCI_CONFIG_READ, N_MK_PCI_CONFIG_WRITE};

pub const MK_PCI_CFG_COMMAND: u32 = 0x04;
pub const MK_PCI_CMD_BUS_MASTER: u16 = 1 << 2;
pub const MK_PCI_MSIX_CTRL_FUNCTION_MASK: u16 = 1 << 14;
pub const MK_PCI_MSIX_CTRL_ENABLE: u16 = 1 << 15;

#[no_mangle]
pub extern "C" fn mk_pci_config_read(
    device_id: u64,
    claim_epoch: u64,
    offset: u32,
    width: u32,
) -> i64 {
    call_raw(N_MK_PCI_CONFIG_READ, [device_id, claim_epoch, offset as u64, width as u64, 0, 0])
}

#[no_mangle]
pub extern "C" fn mk_pci_config_write(
    device_id: u64,
    claim_epoch: u64,
    offset: u32,
    value: u16,
) -> i64 {
    call_raw(N_MK_PCI_CONFIG_WRITE, [device_id, claim_epoch, offset as u64, value as u64, 0, 0])
}
