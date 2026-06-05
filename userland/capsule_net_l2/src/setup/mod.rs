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

mod discover;

pub use discover::{first_available, DiscoverError};

use crate::nic_client::{read_mac, MacError};
use crate::state::STATE;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SetupError {
    NoNic,
    MacFailed,
}

impl From<DiscoverError> for SetupError {
    fn from(_: DiscoverError) -> Self {
        Self::NoNic
    }
}

impl From<MacError> for SetupError {
    fn from(_: MacError) -> Self {
        Self::MacFailed
    }
}

// Resolve the upstream NIC, read its MAC, and install both into
// the capsule state. The IPv4 binding is left zero until DHCP
// completes and the IP capsule pushes it down through the L2
// service interface.
pub fn run() -> Result<(), SetupError> {
    let nic = first_available()?;
    STATE.set_nic(nic.port, nic.pid);
    let mac = read_mac(nic.port)?;
    *STATE.mac.lock() = mac;
    Ok(())
}
