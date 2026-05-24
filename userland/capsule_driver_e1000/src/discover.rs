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

use nonos_libc::{mk_device_list, DeviceRecord, BAR_KIND_MMIO, BUS_KIND_PCI};

use crate::constants::pci::{E1000_DEVICE_IDS, INTEL_VENDOR_ID};

const MAX_DEVICES: usize = 32;
const PCI_CLASS_NETWORK: u8 = 0x02;
const PCI_SUBCLASS_ETHERNET: u8 = 0x00;

#[derive(Debug, Clone, Copy)]
pub struct Found {
    pub device_id: u64,
    pub irq_line: u8,
    pub bar0_size: u64,
}

pub fn find_e1000() -> Option<Found> {
    let mut buf = [DeviceRecord::empty(); MAX_DEVICES];
    let n = mk_device_list(0, buf.as_mut_ptr(), MAX_DEVICES as u64);
    if n <= 0 {
        return None;
    }
    let count = core::cmp::min(n as usize, MAX_DEVICES);
    for r in &buf[..count] {
        if !is_match(r) {
            continue;
        }
        if r.irq_pin == 0 || r.irq_line == 0xFF || r.bar_count == 0 {
            continue;
        }
        let bar0 = r.bars[0];
        if bar0.kind != BAR_KIND_MMIO || bar0.size == 0 {
            continue;
        }
        return Some(Found { device_id: r.device_id, irq_line: r.irq_line, bar0_size: bar0.size });
    }
    None
}

fn is_match(r: &DeviceRecord) -> bool {
    r.vendor == INTEL_VENDOR_ID
        && r.bus_kind == BUS_KIND_PCI
        && E1000_DEVICE_IDS.contains(&r.device)
        && r.pci_class == PCI_CLASS_NETWORK
        && r.pci_subclass == PCI_SUBCLASS_ETHERNET
}
