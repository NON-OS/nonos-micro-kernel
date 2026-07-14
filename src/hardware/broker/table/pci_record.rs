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

use crate::drivers::pci::types::{PciBar, PciDevice};
use crate::hardware::broker::device::{BAR_FLAG_MEM64, BAR_FLAG_PREFETCH};
use crate::hardware::broker::{classify_pci, Bar, BarKind, BusKind, DeviceRecord};

pub(super) fn record_from_pci(device_id: u64, dev: &PciDevice) -> DeviceRecord {
    let mut bars = [Bar::empty(); 6];
    let mut bar_span = 0u8;
    for (i, b) in dev.bars.iter().enumerate() {
        if b.is_present() {
            bars[i] = bar_from_pci(b);
            bar_span = (i + 1) as u8;
        }
    }
    DeviceRecord {
        device_id,
        bus_kind: BusKind::Pci as u8,
        pci_class: dev.class,
        pci_subclass: dev.subclass,
        pci_progif: dev.progif,
        class: classify_pci(dev.class, dev.subclass, dev.progif).id(),
        vendor: dev.vendor_id,
        device: dev.device_id,
        flags: 0,
        bar_count: bar_span,
        irq_line: dev.interrupt_line,
        irq_pin: dev.interrupt_pin,
        _pad1: [0; 1],
        irq_source: dev.interrupt_line as u32,
        bars,
    }
}

fn bar_from_pci(b: &PciBar) -> Bar {
    if b.is_memory() {
        mem_bar(b)
    } else if b.is_io() {
        let port = b.port().unwrap_or(0) as u64;
        Bar { base: port, size: b.size(), kind: BarKind::Pio as u8, flags: 0, aux: 0, _pad: [0; 2] }
    } else {
        Bar::empty()
    }
}

fn mem_bar(b: &PciBar) -> Bar {
    let mut flags = 0u8;
    if b.is_prefetchable() {
        flags |= BAR_FLAG_PREFETCH;
    }
    if b.is_64bit() {
        flags |= BAR_FLAG_MEM64;
    }
    let base = b.address().map(|p| p.as_u64()).unwrap_or(0);
    Bar { base, size: b.size(), kind: BarKind::Mmio as u8, flags, aux: 0, _pad: [0; 2] }
}
