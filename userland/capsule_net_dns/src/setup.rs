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

use nonos_libc::mk_service_lookup;

use crate::state::{local_port, set_udp_port};
use crate::udp_client;

const UDP_NAME: &str = "net.udp";

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SetupError {
    UdpMissing,
    BindFailed,
}

pub fn run() -> Result<(), SetupError> {
    let mut port = 0u32;
    let mut pid = 0u32;
    let rc = mk_service_lookup(UDP_NAME.as_ptr(), UDP_NAME.len(), &mut port, &mut pid);
    if rc != 0 || port == 0 {
        return Err(SetupError::UdpMissing);
    }
    let lport = local_port().ok_or(SetupError::BindFailed)?;
    udp_client::bind(port, lport).map_err(|_| SetupError::BindFailed)?;
    set_udp_port(port);
    crate::dhcp_upstream::apply();
    Ok(())
}
