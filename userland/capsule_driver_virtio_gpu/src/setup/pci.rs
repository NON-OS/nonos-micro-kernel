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

use nonos_libc::{mk_device_release, mk_pci_config_write, MK_PCI_CFG_COMMAND};
use nonos_libc::MK_PCI_CMD_BUS_MASTER;

pub fn enable_bus_master(device_id: u64, claim_epoch: u64) -> Result<(), &'static str> {
    let rc = mk_pci_config_write(device_id, claim_epoch, MK_PCI_CFG_COMMAND, MK_PCI_CMD_BUS_MASTER);
    if rc >= 0 {
        return Ok(());
    }
    if mk_device_release(device_id) < 0 {
        return Err("virtio-gpu: release failed after pci setup failure");
    }
    Err("virtio-gpu: pci bus master enable failed")
}
