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

//! Bounded ACPI AML resource extractor for I2C-HID touchpads.
//!
//! This module scans the DSDT AML byte stream for device objects, matches
//! known touchpad `_HID` values, and parses each device's `_CRS`
//! ResourceTemplate to recover the I2C slave address and GPIO interrupt pin.
//! It is not a full AML interpreter; it never executes AML and returns nothing
//! rather than guessing when the encoding is unexpected.

pub mod controller;
pub mod crs;
mod hid_enumerate;
pub mod scan;
pub mod tables;
pub mod types;

pub use controller::enumerate_i2c_controllers;
pub use hid_enumerate::enumerate_i2c_hid;
pub use types::{I2cHidDevice, LpssController};
