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

//! The HID-over-I2C touchpad driver, proved against a device that speaks the
//! protocol, behind a controller that runs the shipping controller driver.
//!
//! Every `#[path]` below is the shipping i2c_hid source. Its calls into
//! `nonos_libc` land in the shim shared with `i2c_transfer_proofs`: a service
//! lookup finds the controller, an IPC call reaches the shipping OP_TRANSFER
//! handler, and posted input events are kept for the test to read. Below the
//! handler is the modelled DesignWare core with a modelled precision touchpad
//! on its bus. Nothing between the driver's wire encoder and the pad's
//! register file is a stand-in.
//!
//! The server loop and the diagnostics op table are not included: they are
//! the loop around these functions, and the tests call the functions.

extern crate alloc;

#[path = "../../capsule_driver_i2c_hid/src/diag.rs"]
pub mod diag;
#[path = "../../capsule_driver_i2c_hid/src/hid/mod.rs"]
pub mod hid;
#[path = "../../capsule_driver_i2c_hid/src/i2c_client/mod.rs"]
pub mod i2c_client;
#[path = "../../capsule_driver_i2c_hid/src/input/mod.rs"]
pub mod input;
#[path = "../../capsule_driver_i2c_hid/src/setup.rs"]
pub mod setup;
#[path = "../../capsule_driver_i2c_hid/src/state.rs"]
pub mod state;

#[cfg(test)]
mod tests;
