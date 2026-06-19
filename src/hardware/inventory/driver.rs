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

use super::family::HardwareFamily::{self, *};

pub fn family_driver(family: HardwareFamily) -> Option<&'static str> {
    Some(match family {
        StorageNvme => "driver_nvme",
        StorageAhci => "driver_ahci",
        StorageUsbMsc => "driver_usb_msc",
        StorageVirtioBlk => "driver_virtio_blk",
        NetworkVirtio => "driver_virtio_net",
        NetworkE1000 => "driver_e1000",
        NetworkRtl8139 => "driver_rtl8139",
        NetworkRtl8169 => "driver_rtl8169",
        NetworkIwlwifi => "driver_iwlwifi",
        DisplayVirtioGpu => "driver_virtio_gpu",
        DisplayBga => "driver_bga",
        UsbXhci => "driver_xhci",
        InputPs2 => "driver_ps2_input",
        InputUsbHid => "driver_usb_hid",
        InputI2cHid => "driver_i2c_hid",
        AudioHda => "driver_hda",
        SerialI2c => "driver_i2c_pci",
        _ => return None,
    })
}
