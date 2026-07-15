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

use crate::arch::x86_64::acpi::aml::tables;
use crate::arch::x86_64::acpi::aml::types::LpssController;

use super::crs::parse_controller_crs;
use super::find::find_i2c_controller_devices;

/// Enumerate LPSS I2C controllers from the ACPI DSDT/SSDT, returning those with
/// a usable MMIO window. Empty when the platform exposes its controller as a
/// PCI function instead.
pub fn enumerate_i2c_controllers() -> Vec<LpssController> {
    let mut out = Vec::new();
    for aml in tables::aml_blocks() {
        for scope in find_i2c_controller_devices(&aml) {
            let mut ctl = LpssController::new(scope.hid);
            if let Some(crs) = scope.crs {
                parse_controller_crs(crs, &mut ctl);
            }
            if ctl.is_valid() {
                out.push(ctl);
            }
        }
    }
    out
}
