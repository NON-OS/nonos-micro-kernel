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

//! Bring-up. Resolves `net.ip` through `MkServiceLookup`, pulls
//! the current source IPv4 from the IP capsule, and caches both
//! in state so the data path doesn't pay the round-trip later.

use core::sync::atomic::Ordering;

use nonos_libc::mk_service_lookup;

use crate::ip_client::{read_ipv4, ConfigError};
use crate::state::STATE;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SetupError {
    IpNotFound,
    IpConfigFailed,
}

const IP_NAME: &str = "net.ip";

pub fn run() -> Result<(), SetupError> {
    let mut port: u32 = 0;
    let mut pid: u32 = 0;
    let rc = mk_service_lookup(IP_NAME.as_ptr(), IP_NAME.len(), &mut port, &mut pid);
    if rc != 0 {
        return Err(SetupError::IpNotFound);
    }
    STATE.ip_service_port.store(port, Ordering::Release);
    match read_ipv4(port) {
        Ok(addr) => {
            *STATE.local_ipv4.lock() = addr;
            Ok(())
        }
        Err(ConfigError::SendFailed)
        | Err(ConfigError::BadResponse)
        | Err(ConfigError::Refused(_)) => Err(SetupError::IpConfigFailed),
    }
}
