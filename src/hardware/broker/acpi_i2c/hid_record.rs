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

use crate::arch::x86_64::acpi::aml::I2cHidDevice;
use crate::hardware::broker::class::ids;
use crate::hardware::broker::device::{BusKind, DeviceRecord};

/// Build an ACPI-bus DeviceRecord for a firmware-declared I2C-HID touchpad. The
/// slave address rides in `vendor` and the HID descriptor register in `device`
/// so the HID driver binds to the exact device instead of probing a guessed
/// address list; the GPIO interrupt pin rides in `irq_source`, and the
/// firmware-named host controller (as index+1, zero when unnamed) rides in
/// `pci_progif`, which is otherwise unused on an ACPI record.
pub(super) fn hid_record(dev: &I2cHidDevice) -> DeviceRecord {
    DeviceRecord {
        bus_kind: BusKind::Acpi as u8,
        class: ids::I2C_HID,
        vendor: dev.slave_addr,
        device: dev.hid_desc_reg,
        pci_progif: controller_slot(dev.controller),
        irq_source: u32::from(dev.gpio_pin),
        ..DeviceRecord::empty()
    }
}

/// Encode the `_CRS` ResourceSource controller NameSeg ("I2C0".."I2C7") as
/// index+1, zero when absent or not of that shape. The i2c host driver uses it
/// to bind the firmware-named controller when no bus answers the probe, instead
/// of falling back to the first controller that initialises.
fn controller_slot(name: [u8; 4]) -> u8 {
    if name[0..3] == *b"I2C" && name[3].is_ascii_digit() {
        name[3] - b'0' + 1
    } else {
        0
    }
}
