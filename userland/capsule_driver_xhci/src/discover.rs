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
use super::constants::CLASS_USB_HOST_XHCI;
use nonos_libc::{mk_device_list, DeviceRecord, BAR_KIND_MMIO, BUS_KIND_PCI};
const MAX_DEVICES: usize = 32;
const PCI_CLASS_SERIAL_BUS: u8 = 0x0c;
const PCI_SUBCLASS_USB: u8 = 0x03;
const PCI_PROGIF_XHCI: u8 = 0x30;
#[derive(Debug, Clone, Copy)]
pub struct Found {
    pub device_id: u64,
    pub bar0_size: u64,
}
pub fn find_xhci() -> Option<Found> {
    let mut buf = [DeviceRecord::empty(); MAX_DEVICES];
    let n = mk_device_list(0, buf.as_mut_ptr(), MAX_DEVICES as u64);
    if n <= 0 {
        return None;
    }
    let count = core::cmp::min(n as usize, MAX_DEVICES);
    for r in &buf[..count] {
        if r.class != CLASS_USB_HOST_XHCI || !raw_xhci(r) {
            continue;
        }
        if r.bar_count == 0 {
            continue;
        }
        let bar0 = r.bars[0];
        if bar0.kind != BAR_KIND_MMIO || bar0.size == 0 {
            continue;
        }
        return Some(Found { device_id: r.device_id, bar0_size: bar0.size });
    }
    None
}
fn raw_xhci(r: &DeviceRecord) -> bool {
    r.pci_class == PCI_CLASS_SERIAL_BUS
        && r.bus_kind == BUS_KIND_PCI
        && r.pci_subclass == PCI_SUBCLASS_USB
        && r.pci_progif == PCI_PROGIF_XHCI
}
