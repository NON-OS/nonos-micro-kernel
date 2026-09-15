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

//! The driver's register type, assembled from its files.
//!
//! The driver declares these as children of `regs.rs`; included by path,
//! each is named here instead so the sibling `use` lines resolve.

#[path = "../../../capsule_driver_virtio_gpu/src/regs/io.rs"]
mod io;
#[path = "../../../capsule_driver_virtio_gpu/src/regs/notify.rs"]
mod notify;
#[path = "../../../capsule_driver_virtio_gpu/src/regs/pio.rs"]
mod pio;
#[path = "../../../capsule_driver_virtio_gpu/src/regs/read.rs"]
mod read;
#[path = "../../../capsule_driver_virtio_gpu/src/regs/state/mod.rs"]
mod state;
#[path = "../../../capsule_driver_virtio_gpu/src/regs/write.rs"]
mod write;
pub use self::state::Regs;
