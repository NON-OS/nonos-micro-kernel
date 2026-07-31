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

//! The types net_core's lease status decodes to. Splitting "net_core not running"
//! from "running but no address" is the key signal when nothing binds.

/// A bound lease: the address, its prefix length, and the gateway and DNS.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Lease {
    pub ip: [u8; 4],
    pub prefix: u8,
    pub gw: [u8; 4],
    pub dns: [u8; 4],
}

/// What the settings panel resolves for the DHCP client: the two distinct
/// failure modes (service never registered vs registered but not answering) are
/// kept apart so an on-screen reading pinpoints where the query stopped, then
/// running-without-address, then a bound lease on `port`.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum NetStatus {
    /// The `net.dhcp.client` service lookup returned nothing.
    NoService,
    /// The service resolved but the lease-status call got no reply in time.
    NoReply,
    Unbound {
        port: u32,
    },
    Bound {
        lease: Lease,
        port: u32,
    },
}
