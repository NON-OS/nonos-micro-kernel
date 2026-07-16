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

use crate::arch::x86_64::acpi::aml::{
    enumerate_gpio_controllers, enumerate_i2c_controllers, enumerate_i2c_hid,
};

use super::super::table::register_platform_device;
use super::hid_record::hid_record;
use super::record::device_record;

/// Register the ACPI-enumerated LPSS I2C controllers and their I2C-HID
/// touchpads with the broker table. On a machine that exposes its controller as
/// a PCI function the controller loop finds nothing and PCI enumeration covers
/// it; the touchpad records still let the HID driver skip address probing. The
/// GPIO communities are enumerated here as well so each touchpad record can
/// name the community its interrupt pin belongs to.
pub fn register_acpi_i2c() {
    for ctl in enumerate_i2c_controllers() {
        register_platform_device(device_record(&ctl));
    }
    let gpio = enumerate_gpio_controllers();
    for dev in enumerate_i2c_hid() {
        if dev.slave_addr != 0 {
            register_platform_device(hid_record(&dev, &gpio));
        }
    }
}
