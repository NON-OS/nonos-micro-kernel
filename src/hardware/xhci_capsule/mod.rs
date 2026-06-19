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

// Kernel-side glue for the userland xHCI capsule: embed, spawn,
// thin IPC client. Caps = IPC|Memory|Driver|Mmio|Irq|Dma. INTx
// only; MSI/MSI-X is a separate broker work item.

mod capability;
pub mod client;
mod embed;
mod error;
mod protocol;
mod spawn;
mod state;

pub use client::{
    address_device, config_descriptor, controller_status, device_descriptor, healthcheck,
    port_status, AddressedDevice, ControllerStatus, PortSnapshot, CONFIG_DESCRIPTOR_MAX_LEN,
    DEVICE_DESCRIPTOR_LEN,
};
pub use error::DriverXhciError;
pub use spawn::{spawn_driver_xhci_capsule, SpawnError};
pub use state::shared_state;
