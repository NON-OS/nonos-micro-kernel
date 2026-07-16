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

use super::buf::LineBuf;
use super::label::class_label;
use crate::hardware::broker::class::ids;
use crate::hardware::broker::{BusKind, DeviceRecord};
use crate::sys::boot_log;

// Render one device as a single census row. The i2c-HID and i2c controller
// rows carry the fields that decide whether a touchpad can be reached: the
// slave address, its GPIO interrupt, and the controller MMIO base and clock.
pub(super) fn emit(rec: &DeviceRecord) {
    let mut line = LineBuf::new();
    line.put(match rec.bus_kind {
        k if k == BusKind::Pci as u8 => "PCI  ",
        k if k == BusKind::Acpi as u8 => "ACPI ",
        k if k == BusKind::Virt as u8 => "VIRT ",
        _ => "???  ",
    });
    line.put(class_label(rec));
    match rec.class {
        ids::I2C_HID => {
            line.put(" addr=0x");
            line.hex(rec.vendor as u64, 2);
            line.put(" gpio=");
            line.dec(rec.irq_source);
        }
        ids::SERIAL if rec.bus_kind == BusKind::Acpi as u8 => {
            line.put(" mmio=0x");
            line.hex(rec.bars[0].base, 8);
            line.put(" clk=");
            line.dec(rec.bars[0].aux);
        }
        _ => {
            line.put(" ");
            line.hex(rec.vendor as u64, 4);
            line.put(":");
            line.hex(rec.device as u64, 4);
            line.put(" cls=");
            line.hex(rec.pci_class as u64, 2);
            line.put("/");
            line.hex(rec.pci_subclass as u64, 2);
        }
    }
    line.put(" irq=");
    if rec.irq_line == 0xFF {
        line.put("--");
    } else {
        line.dec(rec.irq_line as u32);
    }
    boot_log::info(line.as_str());
}
