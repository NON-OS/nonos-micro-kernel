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

//! Host-runnable proofs for the network capsules' untrusted-input parsers. Each
//! `#[path]` include pulls the real capsule source, so the tests run production
//! parsing code against adversarial packets rather than a copy of it.

extern crate alloc;

// The real DNS module, addressed as `crate::dns` by its own source. The style
// lints are the real code's own choices, allowed on the include rather than
// restyled here.
#[allow(clippy::new_without_default, clippy::unnecessary_map_or)]
#[path = "../../capsule_net_dns/src/dns/mod.rs"]
pub mod dns;

// Minimal mirrors so the ICMP, ARP, TCP and DHCP parsers resolve their siblings.
pub mod arp;
pub mod dhcp;
pub mod ethernet;
pub mod icmp;
pub mod ipv4;
pub mod state;
pub mod tcp;

#[cfg(test)]
mod arp_tests;
#[cfg(test)]
mod dhcp_tests;
#[cfg(test)]
mod dns_tests;
#[cfg(test)]
mod icmp_tests;
#[cfg(test)]
mod tcp_tests;
