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

use crate::arch::x86_64::acpi::aml::enumerate_gpio_controllers;

use super::super::table::register_platform_device;
use super::record::device_record;

/// Register the ACPI-enumerated GPIO community controllers with the broker
/// table. The MMIO grant path hands out whole pages only, so a community whose
/// window is not page-aligned is skipped rather than rounded into neighbouring
/// registers; on the Intel pinctrl family the windows are 64 KiB aligned and
/// this never trips.
pub fn register_acpi_gpio() {
    for ctl in enumerate_gpio_controllers() {
        if ctl.mmio_base & 0xFFF != 0 {
            continue;
        }
        register_platform_device(device_record(&ctl));
    }
}
