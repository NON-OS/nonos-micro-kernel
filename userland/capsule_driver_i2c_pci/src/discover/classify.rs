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
use nonos_libc::{DeviceRecord, BUS_KIND_ACPI, BUS_KIND_PCI};

use super::defs::{ACPI_LPSS_FAMILY, PCI_CLASS_SERIAL_BUS};
use crate::constants::{device_info, INTEL_VENDOR_ID};

// Recognise an LPSS I2C host controller on either bus. A PCI function matches by
// Intel vendor and a known device id; an ACPI device carries its serial-bus
// class and its source clock (in the bar's aux field) from the kernel's AML
// enumeration. The transfer engine polls, so a missing legacy IRQ line never
// disqualifies a controller.
//
// The PCI class is deliberately not gated: Intel LPSS reports the serial-bus
// class (0x0c) on Sunrise Point and later but the signal-processing class (0x11)
// on Apollo/Gemini Lake. The per-device table is the exact filter, so trust it.
pub(super) fn classify(r: &DeviceRecord) -> Option<(&'static str, u32, bool)> {
    if r.bus_kind == BUS_KIND_PCI && r.vendor == INTEL_VENDOR_ID {
        if let Some((family, clock_hz)) = device_info(r.device) {
            return Some((family, clock_hz, false));
        }
    }
    if r.bus_kind == BUS_KIND_ACPI && r.pci_class == PCI_CLASS_SERIAL_BUS {
        return Some((ACPI_LPSS_FAMILY, r.bars[0].aux, true));
    }
    None
}
