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

//! xHCI IPC client for usb_hid. Mirrors `capsule_net_l2/nic_client/`.
//! Call `lookup::lookup()` to resolve the port, then use the typed
//! wrappers from `ops::*`. The generic `call::call` is available for
//! future ops not yet wrapped.

mod call;
mod lookup;
mod ops;
mod seq;
mod wire;

pub use call::XhciClientError;
pub use lookup::lookup;
pub use ops::{
    address_device, AddressedDevice, alloc_transfer_ring, control_transfer,
    enable_slot, get_config_descriptor, interrupt_in, port_status,
    PortSnapshot, MAX_DESCRIPTOR_LEN,
};
