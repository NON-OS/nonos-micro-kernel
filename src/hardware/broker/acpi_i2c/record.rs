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

use crate::arch::x86_64::acpi::aml::LpssController;
use crate::hardware::broker::class::ids;
use crate::hardware::broker::device::{Bar, BarKind, BusKind, DeviceRecord};

use super::clock::source_clock_hz;

// Serial-bus PCI class, mirrored onto the ACPI record so the userland driver
// recognises the controller the same way it does a PCI LPSS function.
const PCI_CLASS_SERIAL_BUS: u8 = 0x0c;

/// Build an ACPI-bus DeviceRecord for a discovered LPSS I2C controller: an MMIO
/// bar over its register window, the polling driver's interrupt, and the source
/// clock carried in the bar's `aux` field for the driver's SCL timing.
pub(super) fn device_record(ctl: &LpssController) -> DeviceRecord {
    let mut bars = [Bar::empty(); 6];
    bars[0] = Bar {
        base: ctl.mmio_base,
        size: u64::from(ctl.mmio_size),
        kind: BarKind::Mmio as u8,
        flags: 0,
        aux: source_clock_hz(&ctl.hid),
        _pad: [0; 2],
    };
    DeviceRecord {
        bus_kind: BusKind::Acpi as u8,
        pci_class: PCI_CLASS_SERIAL_BUS,
        class: ids::SERIAL,
        bar_count: 1,
        irq_line: if ctl.has_irq { ctl.irq as u8 } else { 0xFF },
        bars,
        ..DeviceRecord::empty()
    }
}
