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

//! The virtio-gpu driver, proved without the device.
//!
//! The `#[path]` includes pull in the shipping driver source, so these tests
//! run the code that boots. The register type is built from base addresses,
//! so windows in host memory stand in for the common, notify and device
//! capability regions, and a queue region in host memory stands in for the
//! DMA mapping. The discovery, broker and IPC server halves talk to the
//! kernel rather than to the part and stay out.
//!
//! Two things are held to the specification here. The bring-up: the status
//! handshake, the features the driver accepts against what was offered, and
//! where it tells the part its rings are. The control queue: every command
//! goes out as a two-descriptor chain, the part's answer is read back from
//! the descriptor it named, and the wire bytes of each 2D command match the
//! layouts in the virtio specification, checked by a part on its own thread
//! that consumes the ring the way a device does.

extern crate alloc;

#[path = "../../capsule_driver_virtio_gpu/src/constants/mod.rs"]
pub mod constants;
#[path = "../../capsule_driver_virtio_gpu/src/device/mod.rs"]
pub mod device;
#[path = "../../capsule_driver_virtio_gpu/src/init/mod.rs"]
pub mod init;
#[path = "../../capsule_driver_virtio_gpu/src/protocol/mod.rs"]
pub mod protocol;
pub mod regs;

#[cfg(test)]
mod tests;
