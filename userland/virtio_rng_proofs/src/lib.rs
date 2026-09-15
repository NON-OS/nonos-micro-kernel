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

//! The virtio-rng bring-up handshake, proved against a register window rather
//! than against a device.
//!
//! Each `#[path]` below pulls in the shipping driver source, so these tests
//! run the code that boots, not a copy of it. Before this crate the driver had
//! no host coverage at all: every line of it was reachable only by booting a
//! machine and hoping the device showed up.
//!
//! The handshake is worth proving without hardware because the virtio
//! specification, not the device, is what fixes it. The status byte must be
//! driven reset, ACKNOWLEDGE, DRIVER, FEATURES_OK, DRIVER_OK in that order,
//! and a device that is told DRIVER_OK before its queue is configured is
//! entitled to do anything at all. Getting that order wrong is wrong against
//! every conforming implementation, which is exactly the class of defect a
//! test with no device can catch.

#[path = "../../capsule_driver_virtio_rng/src/constants/mod.rs"]
pub mod constants;

pub mod regs;

#[path = "../../capsule_driver_virtio_rng/src/init.rs"]
pub mod init;

#[cfg(test)]
mod tests;
