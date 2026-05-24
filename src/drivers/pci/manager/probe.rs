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

extern crate alloc;

use alloc::vec::Vec;

use super::super::bar::decode_all_bars_unchecked;
use super::super::capabilities::{
    collect_all_capabilities, enumerate_pcie_capabilities, get_msi_info, get_msix_info,
    get_pcie_info, get_power_management_info, get_virtio_info,
};
use super::super::config::{read32_unchecked, ConfigSpace};
use super::super::constants::*;
use super::super::security::{check_device_allowed, is_dma_capable};
use super::super::stats;
use super::super::types::{ClassCode, DeviceId, HeaderType, PciAddress, PciDevice};

pub(super) fn probe_device(bus: u8, device_num: u8, function: u8) -> Option<PciDevice> {
    let id = read32_unchecked(bus, device_num, function, CFG_VENDOR_ID as u8);
    let vendor_id = (id & 0xFFFF) as u16;

    if vendor_id == 0xFFFF || vendor_id == 0x0000 {
        return None;
    }

    let device_id = ((id >> 16) & 0xFFFF) as u16;

    if check_device_allowed(vendor_id, device_id).is_err() {
        return None;
    }

    let class_reg = read32_unchecked(bus, device_num, function, CFG_REVISION_ID as u8);
    let revision = (class_reg & 0xFF) as u8;
    let prog_if = ((class_reg >> 8) & 0xFF) as u8;
    let subclass = ((class_reg >> 16) & 0xFF) as u8;
    let class = ((class_reg >> 24) & 0xFF) as u8;

    let header_reg = read32_unchecked(bus, device_num, function, CFG_CACHE_LINE_SIZE as u8);
    let header_type_raw = ((header_reg >> 16) & 0xFF) as u8;
    let multifunction = (header_type_raw & HDR_TYPE_MULTIFUNCTION) != 0;
    let header_type = HeaderType::from(header_type_raw);

    let subsys_reg = read32_unchecked(bus, device_num, function, CFG_SUBSYSTEM_VENDOR_ID as u8);
    let subsystem_vendor_id = (subsys_reg & 0xFFFF) as u16;
    let subsystem_id = ((subsys_reg >> 16) & 0xFFFF) as u16;

    let int_reg = read32_unchecked(bus, device_num, function, CFG_INTERRUPT_LINE as u8);
    let interrupt_line = (int_reg & 0xFF) as u8;
    let interrupt_pin = ((int_reg >> 8) & 0xFF) as u8;

    let address = PciAddress::new(bus, device_num, function);
    let config = ConfigSpace::new(address);

    let bars = decode_all_bars_unchecked(bus, device_num, function);

    let capabilities = collect_all_capabilities(&config).unwrap_or_default();
    let pcie_capabilities = enumerate_pcie_capabilities(bus, device_num, function);

    let msi = get_msi_info(&config).ok().flatten();
    let msix = get_msix_info(&config).ok().flatten();
    let virtio = get_virtio_info(&config).ok().flatten();
    let power_management = get_power_management_info(&config).ok().flatten();
    let pcie = get_pcie_info(&config).ok().flatten();

    let mut device = PciDevice::new(address);
    device.device_id_info =
        DeviceId { vendor_id, device_id, subsystem_vendor_id, subsystem_id, revision };
    device.class_code = ClassCode::new(class, subclass, prog_if);
    device.header_type = header_type;
    device.multifunction = multifunction;
    device.bars = bars;
    device.capabilities = capabilities;
    device.pcie_capabilities = pcie_capabilities;
    device.interrupt_line = interrupt_line;
    device.interrupt_pin = interrupt_pin;
    device.msi = msi;
    device.msix = msix;
    device.virtio = virtio;
    device.power_management = power_management;
    device.pcie = pcie;

    device.sync_compat_fields();

    stats::record_device_found(
        class,
        vendor_id,
        device.is_bridge(),
        device.is_pcie(),
        device.supports_msi(),
        device.supports_msix(),
        is_dma_capable(&device),
    );

    Some(device)
}

pub(super) fn enumerate_bus(bus: u8, devices: &mut Vec<PciDevice>) {
    for device_num in 0..32u8 {
        let id = read32_unchecked(bus, device_num, 0, CFG_VENDOR_ID as u8);
        let vendor_id = (id & 0xFFFF) as u16;

        if vendor_id == 0xFFFF || vendor_id == 0x0000 {
            continue;
        }

        let header_reg = read32_unchecked(bus, device_num, 0, CFG_CACHE_LINE_SIZE as u8);
        let header_type = ((header_reg >> 16) & 0xFF) as u8;
        let multifunction = (header_type & HDR_TYPE_MULTIFUNCTION) != 0;

        if let Some(dev) = probe_device(bus, device_num, 0) {
            devices.push(dev);
        }

        if multifunction {
            for function in 1..8u8 {
                if let Some(dev) = probe_device(bus, device_num, function) {
                    devices.push(dev);
                }
            }
        }
    }
}

pub(super) fn enumerate_all_buses() -> Vec<PciDevice> {
    let mut devices = Vec::new();

    for bus in 0..=255u8 {
        enumerate_bus(bus, &mut devices);
    }

    devices
}
