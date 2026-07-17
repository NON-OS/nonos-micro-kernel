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

pub mod capabilities;
pub mod link_up;
pub mod mac;
pub mod nic_device;
pub mod read_mac;
pub mod receive;
pub mod rx;
pub mod rx_seq;
pub mod rx_token;
pub mod transmit;
pub mod tx;
pub mod tx_seq;
pub mod tx_token;
pub mod types;

pub use link_up::link_up;
pub use read_mac::mac;
pub use types::NicDevice;

/// The timeout for a single request to a NIC driver. Bounded and short so a slow
/// or dead interface (a cable-less wired NIC that answers link-up but never
/// carries traffic) can never stall the poll loop, keeping the stack responsive
/// and free to re-evaluate onto the WiFi link the moment it associates.
pub const DEVICE_CALL_MS: u64 = 250;
