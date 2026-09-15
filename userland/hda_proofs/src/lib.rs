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

//! Intel HD Audio controller bring-up, proved without a controller.
//!
//! The `#[path]` includes below pull in the shipping driver source, so these
//! tests run the code that boots. Most of this capsule only answers when
//! something is driving the other end of the ring, which is why it had no host
//! coverage at all. `nonos_devmodel::run` puts that something on a thread.
//!
//! What is provable here is what the HD Audio specification fixes in writing:
//! which registers a conforming controller requires, holding what, and that
//! every wait on a codec gives up rather than wedging boot. Timing is not
//! modelled, and neither are the halves of the capsule that talk to the broker
//! rather than to registers (`discover`, `setup`, `server`, `handles`).

#[path = "../../capsule_driver_hda/src/regs/mod.rs"]
pub mod regs;

#[path = "../../capsule_driver_hda/src/constants/mod.rs"]
pub mod constants;

#[path = "../../capsule_driver_hda/src/error/mod.rs"]
pub mod error;

#[cfg(test)]
pub mod controller;

#[cfg(test)]
mod model;

#[cfg(test)]
mod proofs;
