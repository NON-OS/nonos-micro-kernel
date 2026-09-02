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

use super::value::Metric;

pub const IFNAME_LEN: usize = 8;

const NET0: [u8; IFNAME_LEN] = *b"net0\0\0\0\0";

/// The one interface NONOS brings up is the virtio-net device the DHCP client
/// leases on. Address and prefix come off that lease; there is no v6 stack and
/// the driver exposes no byte counters, so those rows have no source at all.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Net {
    pub name: [u8; IFNAME_LEN],
    pub name_len: u8,
    pub up: bool,
    pub ipv4: Metric<[u8; 4]>,
    pub prefix_len: Metric<u8>,
    pub gateway: Metric<[u8; 4]>,
    pub ipv6: Metric<[u8; 16]>,
    pub rx_bps: Metric<u64>,
    pub tx_bps: Metric<u64>,
}

impl Net {
    pub const DOWN: Net = Net {
        name: NET0,
        name_len: 4,
        up: false,
        ipv4: Metric::Unknown,
        prefix_len: Metric::Unknown,
        gateway: Metric::Unknown,
        ipv6: Metric::Unsupported,
        rx_bps: Metric::Unsupported,
        tx_bps: Metric::Unsupported,
    };

    pub fn name_str(&self) -> &str {
        let n = (self.name_len as usize).min(IFNAME_LEN);
        core::str::from_utf8(&self.name[..n]).unwrap_or("")
    }
}
