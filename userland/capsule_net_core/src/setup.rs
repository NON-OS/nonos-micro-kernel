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

use crate::device;
use crate::iface::build;
use crate::state;

const NIC_CANDIDATES: &[&str] =
    &["driver.virtio_net0", "driver.e1000_0", "driver.rtl8169_0", "driver.rtl8139_0"];

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SetupError {
    NicNotFound,
    LinkDown,
    LinkFailed,
    MacFailed,
    BuildFailed,
}

fn discover_nic() -> Option<u32> {
    for name in NIC_CANDIDATES {
        let mut port: u32 = 0;
        let mut pid: u32 = 0;
        if mk_service_lookup(name.as_ptr(), name.len(), &mut port, &mut pid) == 0 {
            return Some(port);
        }
    }
    None
}

pub fn run() -> Result<(), SetupError> {
    let port = discover_nic().ok_or(SetupError::NicNotFound)?;
    let link_up = device::link_up(port).ok_or(SetupError::LinkFailed)?;
    if !link_up {
        return Err(SetupError::LinkDown);
    }
    let mac = device::mac(port).ok_or(SetupError::MacFailed)?;
    let net_state = build::build(mac, port).ok_or(SetupError::BuildFailed)?;
    state::store(net_state);
    Ok(())
}
