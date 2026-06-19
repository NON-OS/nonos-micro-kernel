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

//! Kernel-side glue for the PS/2 keyboard userland driver capsule.
//! Embed, spawn, and a thin IPC client that mirrors the userland
//! endpoint surface (healthcheck, poll_events, get_state). The
//! kernel does not touch the i8042 — the capsule speaks broker
//! syscalls (PIO + IRQ) and owns the controller.

mod capability;
pub mod client;
mod embed;
mod error;
mod protocol;
mod spawn;
mod state;

pub use client::{get_state, healthcheck, poll_events, KeyEvent, RingState};
pub use error::DriverPs2Error;
pub use spawn::{spawn_driver_ps2_input_capsule, SpawnError};
pub use state::shared_state;
