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

//! Capsule bring-up. Resolves `net.l2` through `MkServiceLookup`,
//! reads the underlying NIC's MAC, and seeds the interface config
//! with the MAC + a zero IPv4 (DHCP fills the IPv4 later through
//! the OP_SET_CONFIG entry on `net.ip`).

use nonos_libc::mk_service_lookup;

use crate::l2_client::{read_mac, MacError};
use crate::state::IFACE;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SetupError {
    L2NotFound,
    L2MacFailed,
}

const L2_NAME: &str = "net.l2";

pub fn run() -> Result<(), SetupError> {
    let mut port: u32 = 0;
    let mut pid: u32 = 0;
    let rc = mk_service_lookup(L2_NAME.as_ptr(), L2_NAME.len(), &mut port, &mut pid);
    if rc != 0 {
        return Err(SetupError::L2NotFound);
    }
    IFACE.l2_service_port.store(port, core::sync::atomic::Ordering::Release);
    match read_mac(port) {
        Ok(mac) => {
            *IFACE.mac.lock() = mac;
            Ok(())
        }
        Err(MacError::SendFailed) | Err(MacError::BadResponse) | Err(MacError::L2Refused) => {
            Err(SetupError::L2MacFailed)
        }
    }
}
