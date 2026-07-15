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

use crate::constants::{AHCI_ABAR_BAR, CLASS_BLOCK};

const MAX_DEVICES: usize = 32;
const MIN_ABAR_BYTES: u64 = 0x1000;
const PCI_CLASS_STORAGE: u8 = 0x01;
const PCI_SUBCLASS_SATA: u8 = 0x06;
const PCI_PROGIF_AHCI: u8 = 0x01;

#[derive(Debug, Clone, Copy)]
pub struct Found {
    pub device_id: u64,
    pub irq_line: u8,
    pub abar_size: u64,
}

pub fn find_ahci() -> Option<Found> {
    let mut buf = [DeviceRecord::empty(); MAX_DEVICES];
    let n = mk_device_list(CLASS_BLOCK, buf.as_mut_ptr(), MAX_DEVICES as u64);
    if n <= 0 {
        return None;
    }
    for r in &buf[..core::cmp::min(n as usize, MAX_DEVICES)] {
        if is_candidate(r) {
            return Some(Found {
                device_id: r.device_id,
                irq_line: r.irq_line,
                abar_size: r.bars[AHCI_ABAR_BAR as usize].size,
            });
        }
    }
    None
}

fn is_candidate(r: &DeviceRecord) -> bool {
    let bar = r.bars[AHCI_ABAR_BAR as usize];
    r.bus_kind == BUS_KIND_PCI
        && r.class == CLASS_BLOCK
        && r.pci_class == PCI_CLASS_STORAGE
        && r.pci_subclass == PCI_SUBCLASS_SATA
        && r.pci_progif == PCI_PROGIF_AHCI
        && r.bar_count > AHCI_ABAR_BAR
        // The IRQ routing is intentionally not part of the match. The I/O
        // path polls the port completion registers and never waits on the
        // interrupt, so a controller with no legacy PIC routing (irq_line
        // reported as 0xff, common on APIC/MSI laptop platforms) is still a
        // valid candidate. Requiring a routed line here rejected working
        // SATA controllers on real hardware.
        && bar.kind == BAR_KIND_MMIO
        && bar.size >= MIN_ABAR_BYTES
}
