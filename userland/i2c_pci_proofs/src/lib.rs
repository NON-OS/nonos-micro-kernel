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

//! Intel LPSS I2C bring-up, proved against a register window rather than
//! against a controller.
//!
//! Each `#[path]` below pulls in the shipping driver source, so these tests
//! run the code that boots, not a copy of it. This capsule carries the
//! touchpad on every Intel laptop this system targets, and until now a line
//! of its bring-up could only be run by booting one of those laptops.
//!
//! What is provable here is what the DesignWare databook and Intel's LPSS
//! wrapper fix in writing: which register proves MMIO is alive, which reset
//! must be deasserted first, and which registers are read-only once the core
//! is enabled. Getting those wrong breaks every part that implements them.

#[path = "../../capsule_driver_i2c_pci/src/constants/mod.rs"]
pub mod constants;

#[path = "../../capsule_driver_i2c_pci/src/regs.rs"]
pub mod regs;

#[path = "../../capsule_driver_i2c_pci/src/protocol/mod.rs"]
pub mod protocol;

#[path = "../../capsule_driver_i2c_pci/src/transaction/mod.rs"]
pub mod transaction;

#[path = "../../capsule_driver_i2c_pci/src/init/mod.rs"]
pub mod init;

#[path = "../../capsule_driver_i2c_pci/src/driver.rs"]
pub mod driver;

#[cfg(test)]
mod clock_tests;
#[cfg(test)]
mod enable_state_tests;
#[cfg(test)]
mod mmio_tests;
#[cfg(test)]
mod model;
#[cfg(test)]
mod reset_tests;
#[cfg(test)]
mod scl_count_tests;
#[cfg(test)]
mod scl_tests;
#[cfg(test)]
mod target_tests;
