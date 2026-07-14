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

use super::class::ids;
use super::device::{Bar, BarKind, BusKind, DeviceRecord};
use super::table;

pub(super) const PNP_VENDOR_PS2_KBD: u16 = 0x0001;
pub(super) const PNP_DEVICE_PS2_KBD: u16 = 0x0303;
pub(super) const PNP_VENDOR_PS2_AUX: u16 = 0x0001;
pub(super) const PNP_DEVICE_PS2_AUX: u16 = 0x0304;

const PS2_PORT_BASE: u64 = 0x60;
const PS2_PORT_COUNT: u64 = 5;
const PS2_KBD_IRQ: u8 = 1;
const PS2_AUX_IRQ: u8 = 12;

// Fabricate the legacy PS/2 keyboard and aux (mouse) device records.
//
// The i8042 is not enumerable, so we synthesize its records for the driver
// broker. The FADT IA-PC HAS_8042 boot flag is advisory and unreliable in
// practice: real laptops (for example HP Gemini Lake, whose keyboard is an
// EC-emulated i8042) clear it while still exposing a working controller. We
// therefore always fabricate the records and let the PS/2 driver decide: its
// i8042 config read returns 0xFF and it bails cleanly on a machine that truly
// has no controller. Returns the keyboard record id.
pub fn register_legacy() -> u64 {
    let mut bars = [Bar::empty(); 6];
    bars[0] = Bar {
        base: PS2_PORT_BASE,
        size: PS2_PORT_COUNT,
        kind: BarKind::Pio as u8,
        flags: 0,
        aux: 0,
        _pad: [0; 2],
    };
    let kbd = DeviceRecord {
        bus_kind: BusKind::Acpi as u8,
        class: ids::INPUT,
        vendor: PNP_VENDOR_PS2_KBD,
        device: PNP_DEVICE_PS2_KBD,
        bar_count: 1,
        irq_line: PS2_KBD_IRQ,
        irq_pin: 1,
        bars,
        ..DeviceRecord::empty()
    };
    let kbd_id = table::register_platform_device(kbd);

    let aux = DeviceRecord {
        bus_kind: BusKind::Acpi as u8,
        class: ids::INPUT,
        vendor: PNP_VENDOR_PS2_AUX,
        device: PNP_DEVICE_PS2_AUX,
        irq_line: PS2_AUX_IRQ,
        irq_pin: 1,
        ..DeviceRecord::empty()
    };
    table::register_platform_device(aux);
    kbd_id
}
