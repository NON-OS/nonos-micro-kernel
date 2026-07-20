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

//! Discover the WiFi adapters present so the panel can list them before any
//! network scan. A WiFi controller is recognised by PCI class alone, not by
//! brand: it is a network-class device (0x02) with the "other network
//! controller" subclass (0x80), which is how wireless NICs are classed while
//! wired Ethernet uses subclass 0x00. Each adapter carries a friendly name
//! derived from its vendor and device id for display, but selection keys off
//! the stable device id, never the name, so an unknown-but-valid card still
//! lists and stays selectable.

/// PCI base class for network controllers.
pub const PCI_CLASS_NETWORK: u8 = 0x02;
/// PCI subclass for "other" network controllers, which is where wireless NICs
/// report (Ethernet is subclass 0x00).
pub const PCI_SUBCLASS_WIRELESS: u8 = 0x80;
/// Bus kind tag for a PCI device, as `mk_device_list` reports it.
pub const BUS_KIND_PCI: u8 = 1;

/// The most an adapter name occupies, in bytes.
pub const NAME_MAX: usize = 32;

/// The minimal device facts the panel needs to recognise and list an adapter.
/// This mirrors the fields of the broker `DeviceRecord` the panel reads, kept
/// separate so the discovery logic stays pure and host-testable.
#[derive(Clone, Copy)]
pub struct DeviceView {
    pub device_id: u64,
    pub bus_kind: u8,
    pub pci_class: u8,
    pub pci_subclass: u8,
    pub vendor: u16,
    pub device: u16,
}

/// One discovered WiFi adapter as the panel lists it.
#[derive(Clone, Copy, Default)]
pub struct WifiInterface {
    /// The broker handle used to claim and drive the device.
    pub device_id: u64,
    pub vendor: u16,
    pub device: u16,
    name: [u8; NAME_MAX],
    name_len: usize,
}

impl WifiInterface {
    /// The friendly adapter name for display.
    pub fn name(&self) -> &[u8] {
        &self.name[..self.name_len]
    }
}

/// True when a device is a WiFi controller, judged by PCI class alone.
pub fn is_wifi(d: &DeviceView) -> bool {
    d.bus_kind == BUS_KIND_PCI
        && d.pci_class == PCI_CLASS_NETWORK
        && d.pci_subclass == PCI_SUBCLASS_WIRELESS
}

/// Fill `out` with the WiFi adapters found in `devices`, returning how many
/// were written. Extra devices past `out`'s capacity are dropped rather than
/// overrun.
pub fn discover(devices: &[DeviceView], out: &mut [WifiInterface]) -> usize {
    let mut n = 0;
    for d in devices {
        if n == out.len() {
            break;
        }
        if is_wifi(d) {
            out[n] = interface_of(d);
            n += 1;
        }
    }
    n
}

fn interface_of(d: &DeviceView) -> WifiInterface {
    let mut name = [0u8; NAME_MAX];
    let label = adapter_label(d.vendor, d.device);
    let len = label.len().min(NAME_MAX);
    name[..len].copy_from_slice(&label.as_bytes()[..len]);
    WifiInterface {
        device_id: d.device_id,
        vendor: d.vendor,
        device: d.device,
        name,
        name_len: len,
    }
}

/// A friendly name from vendor and device id. Known Intel and Realtek WiFi
/// parts get a specific label; any other WiFi-class card gets a generic vendor
/// label so it still lists.
pub fn adapter_label(vendor: u16, device: u16) -> &'static str {
    const INTEL: u16 = 0x8086;
    const REALTEK: u16 = 0x10EC;
    match (vendor, device) {
        (REALTEK, 0xC821) => "Realtek RTL8821CE",
        (INTEL, 0x2723 | 0x02F0 | 0x06F0 | 0x34F0 | 0x3DF0 | 0x4DF0 | 0x43F0) => {
            "Intel Wi-Fi 6 AX200"
        }
        (INTEL, 0x2725 | 0x2726 | 0x51F0 | 0x51F1 | 0x54F0 | 0x7AF0 | 0x7E40) => {
            "Intel Wi-Fi 6E AX210"
        }
        (INTEL, _) => "Intel Wi-Fi",
        (REALTEK, _) => "Realtek Wi-Fi",
        _ => "Wi-Fi adapter",
    }
}
