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

mod address_device;
mod alloc_transfer_ring;
mod config_descriptor;
mod control_transfer;
mod enable_slot;
mod interrupt_in;
mod port_status;

pub use address_device::{address_device, AddressedDevice};
pub use alloc_transfer_ring::alloc_transfer_ring;
pub use config_descriptor::{get_config_descriptor, MAX_DESCRIPTOR_LEN};
pub use control_transfer::control_transfer;
pub use enable_slot::enable_slot;
pub use interrupt_in::interrupt_in;
pub use port_status::{port_status, PortSnapshot};
