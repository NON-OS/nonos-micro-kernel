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

use nonos_libc::{crypto_random, mk_time_millis};
use smoltcp::iface::{Config, Interface, SocketSet};
use smoltcp::time::Instant;
use smoltcp::wire::{EthernetAddress, HardwareAddress};

use crate::device::NicDevice;
use crate::iface::dhcp;
use crate::state::NetState;

pub fn build(mac: [u8; 6], port: u32) -> Option<NetState> {
    let mut seed_bytes = [0u8; 8];
    if crypto_random(seed_bytes.as_mut_ptr(), 8) != 8 {
        return None;
    }
    let seed = u64::from_le_bytes(seed_bytes);

    let mut config = Config::new(HardwareAddress::Ethernet(EthernetAddress(mac)));
    config.random_seed = seed;

    let now = Instant::from_millis(mk_time_millis());
    let mut device = NicDevice { port };
    let iface = Interface::new(config, &mut device, now);

    let mut sockets = SocketSet::new(alloc::vec![]);
    let dhcp_handle = dhcp::create(&mut sockets);

    Some(NetState { iface, sockets, device, dhcp_handle })
}
