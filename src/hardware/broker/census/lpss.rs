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

use crate::hardware::broker::{BusKind, DeviceRecord};

const INTEL_VENDOR: u16 = 0x8086;
const PCI_CLASS_SERIAL_BUS: u8 = 0x0c;
const PCI_CLASS_SIGNAL_PROC: u8 = 0x11;
const PCI_SUBCLASS_OTHER: u8 = 0x80;

// An Intel LPSS controller function. LPSS reports the serial-bus class (0x0c) on
// Sunrise Point and later but the signal-processing class (0x11) on
// Apollo/Gemini Lake; both use the "other" subclass. Counting both keeps this
// honest on Gemini Lake, where the real I2C hosts are class 0x11 and the broker
// files them under its catch-all "other" class label.
pub(super) fn is_lpss_i2c(rec: &DeviceRecord) -> bool {
    rec.bus_kind == BusKind::Pci as u8
        && rec.vendor == INTEL_VENDOR
        && rec.pci_subclass == PCI_SUBCLASS_OTHER
        && matches!(rec.pci_class, PCI_CLASS_SERIAL_BUS | PCI_CLASS_SIGNAL_PROC)
}
