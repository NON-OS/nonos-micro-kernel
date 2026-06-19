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

//! Kernel-side glue for the AHCI userland driver capsule.
//! The kernel embeds and spawns the signed controller capsule, then
//! speaks the controller inventory and block I/O IPC contract
//! (capacity, read_blocks, write_blocks, flush). ABAR mapping,
//! AHCI-mode enable, the command engine, IRQ acking, and port
//! register access stay inside `driver.ahci0`.

mod capability;
pub mod client;
mod embed;
mod error;
mod protocol;
mod spawn;
mod state;

pub use client::{
    capacity, controller_info, flush, healthcheck, port_list, read_blocks, write_blocks,
    AhciControllerInfo, AhciPortInfo,
};
pub use error::DriverAhciError;
pub use spawn::{spawn_driver_ahci_capsule, SpawnError};
pub use state::shared_state;
