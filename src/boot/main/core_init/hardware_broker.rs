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

use crate::sys::serial;

pub(super) fn seed_hardware_broker() {
    let devices = match crate::drivers::pci::manager::scan_and_collect_safe() {
        Ok(v) => v,
        Err(_) => {
            serial::println(b"[NONOS] PCI scan failed; broker table empty");
            return;
        }
    };
    crate::hardware::broker::init_from_pci(&devices);
    let _ = crate::hardware::broker::register_legacy_platform_devices();
    // Firmware-described I2C and GPIO. ACPI names them in AML and only x86
    // firmware speaks it; an ARM board carries the same controllers in its
    // device tree, which the arch layer walks separately.
    #[cfg(target_arch = "x86_64")]
    {
        crate::hardware::broker::register_acpi_i2c();
        crate::hardware::broker::register_acpi_gpio();
    }
    serial::println(b"[NONOS] hardware broker seeded");
}
