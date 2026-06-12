// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use crate::firmware::types::FirmwareType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
    WiFi,
    Ethernet,
    Gpu,
    Audio,
    Storage,
    Unknown,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProbeResult {
    Detected,
    NotFound,
    Error,
    Incompatible,
}
#[derive(Debug, Clone)]
pub struct HardwareDevice {
    pub device_type: DeviceType,
    pub vendor_id: u16,
    pub device_id: u16,
    pub subsystem_id: u16,
    pub revision: u8,
    pub firmware_type: FirmwareType,
}

pub fn detect_hardware_devices() -> alloc::vec::Vec<HardwareDevice> {
    let mut devices = alloc::vec::Vec::new();
    for bus in 0..8u8 {
        for dev in 0..32u8 {
            for func in 0..8u8 {
                if let Some(hw) = probe_pci(bus, dev, func) {
                    devices.push(hw);
                }
            }
        }
    }
    devices
}

pub fn probe_device(vendor_id: u16, device_id: u16) -> ProbeResult {
    if vendor_id == 0xFFFF || device_id == 0xFFFF {
        return ProbeResult::NotFound;
    }
    if classify(vendor_id, device_id) == DeviceType::Unknown {
        return ProbeResult::Incompatible;
    }
    if map_fw(vendor_id, device_id) != FirmwareType::Unknown {
        ProbeResult::Detected
    } else {
        ProbeResult::Incompatible
    }
}

fn probe_pci(bus: u8, dev: u8, func: u8) -> Option<HardwareDevice> {
    let addr =
        0x80000000 | (u32::from(bus) << 16) | (u32::from(dev) << 11) | (u32::from(func) << 8);
    let vd = pci_read(addr);
    if (vd & 0xFFFF) == 0xFFFF {
        return None;
    }
    let (vendor_id, device_id) = ((vd & 0xFFFF) as u16, (vd >> 16) as u16);
    let revision = (pci_read(addr + 8) & 0xFF) as u8;
    let subsystem_id = (pci_read(addr + 44) >> 16) as u16;
    Some(HardwareDevice {
        device_type: classify(vendor_id, device_id),
        vendor_id,
        device_id,
        subsystem_id,
        revision,
        firmware_type: map_fw(vendor_id, device_id),
    })
}

fn classify(vid: u16, did: u16) -> DeviceType {
    match vid {
        0x8086 => match did {
            0x24FD..=0x24FF => DeviceType::WiFi,
            0x15B7..=0x15B9 => DeviceType::Ethernet,
            0x1912..=0x1932 => DeviceType::Gpu,
            _ => DeviceType::Unknown,
        },
        0x10EC => DeviceType::WiFi,
        0x1002 | 0x10DE => DeviceType::Gpu,
        _ => DeviceType::Unknown,
    }
}

fn map_fw(vid: u16, did: u16) -> FirmwareType {
    match (vid, did) {
        (0x8086, 0x24FD) => FirmwareType::IntelAx200,
        (0x8086, 0x24F3) => FirmwareType::IntelAx210,
        (0x8086, 0x24F4) => FirmwareType::Intel8265,
        (0x10EC, 0xC821) => FirmwareType::Rtl8821c,
        (0x10EC, 0xC822) => FirmwareType::Rtl8822c,
        _ => FirmwareType::Unknown,
    }
}

fn pci_read(addr: u32) -> u32 {
    let data: u32;
    unsafe {
        core::arch::asm!(
            "out dx, eax",
            in("dx") 0xCF8u16,
            in("eax") addr & 0xFFFF_FFFC,
            options(nomem, nostack, preserves_flags)
        );
        core::arch::asm!(
            "in eax, dx",
            out("eax") data,
            in("dx") 0xCFCu16,
            options(nomem, nostack, preserves_flags)
        );
    }
    data
}
