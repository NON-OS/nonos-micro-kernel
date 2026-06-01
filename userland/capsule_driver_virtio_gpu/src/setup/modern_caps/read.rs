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

use nonos_libc::mk_pci_config_read;

pub fn u8_at(device_id: u64, epoch: u64, off: u32) -> Result<u8, &'static str> {
    let rc = mk_pci_config_read(device_id, epoch, off, 1);
    if rc < 0 {
        return Err("virtio-gpu: pci config read8 failed");
    }
    Ok(rc as u8)
}

pub fn u32_at(device_id: u64, epoch: u64, off: u32) -> Result<u32, &'static str> {
    let rc = mk_pci_config_read(device_id, epoch, off, 4);
    if rc < 0 {
        return Err("virtio-gpu: pci config read32 failed");
    }
    Ok(rc as u32)
}
