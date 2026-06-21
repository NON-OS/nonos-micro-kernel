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

//! Kernel-side glue for the virtio-blk userland driver capsule.
//! Embed, spawn, and a thin IPC client that mirrors the userland
//! endpoint surface (healthcheck, capacity, read_blocks,
//! write_blocks, flush). The kernel does not touch the device —
//! the capsule speaks broker syscalls and owns the queue.

mod capability;
pub mod client;
mod embed;
mod error;
mod protocol;
mod spawn;
mod state;

pub use client::{capacity, flush, healthcheck, read_blocks, write_blocks};
pub use error::DriverBlkError;
pub use spawn::{spawn_driver_virtio_blk_capsule, SpawnError};
pub use state::shared_state;
