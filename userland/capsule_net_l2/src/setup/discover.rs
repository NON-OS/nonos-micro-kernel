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

// Names the L2 capsule probes for, in order. First one the
// kernel registry knows wins. Adding a NIC class is one line —
// the data plane upstream of L2 sees no difference.
const NIC_CANDIDATES: &[&str] =
    &["driver.virtio_net0", "driver.e1000_0", "driver.rtl8169_0", "driver.rtl8139_0"];

#[derive(Clone, Copy)]
pub struct Nic {
    pub port: u32,
    pub pid: u32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DiscoverError {
    NotFound,
}

pub fn first_available() -> Result<Nic, DiscoverError> {
    for name in NIC_CANDIDATES {
        let mut port: u32 = 0;
        let mut pid: u32 = 0;
        let rc = mk_service_lookup(name.as_ptr(), name.len(), &mut port, &mut pid);
        if rc == 0 {
            return Ok(Nic { port, pid });
        }
    }
    Err(DiscoverError::NotFound)
}
