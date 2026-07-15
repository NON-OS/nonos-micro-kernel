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

use alloc::vec::Vec;

use super::types::I2cHidDevice;
use super::{crs, scan, tables};

/// Enumerate I2C-HID touchpad devices from the ACPI DSDT/SSDT, each carrying the
/// firmware-declared slave address and GPIO interrupt from its `_CRS`. Empty on
/// any failure; never panics.
pub fn enumerate_i2c_hid() -> Vec<I2cHidDevice> {
    let mut devices = Vec::new();
    for aml in tables::aml_blocks() {
        for scope in scan::find_touchpad_devices(&aml) {
            let mut dev = I2cHidDevice::new(scope.hid);
            if let Some(crs_value) = scope.crs {
                crs::parse_crs(crs_value, &mut dev);
            }
            // A `_CRS` method that concatenates named templates leaves the
            // descriptors out of the method body; they sit in the device body as
            // `Name (SBFB, ResourceTemplate)` buffers. Scan the whole body when
            // the scoped parse did not resolve the address or interrupt.
            if dev.slave_addr == 0 || !dev.has_gpio {
                crs::parse_crs(scope.body, &mut dev);
            }
            devices.push(dev);
        }
    }
    devices
}
