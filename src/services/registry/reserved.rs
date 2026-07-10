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

//! Reserved core-service names and ports. The trusted core services (keyring,
//! crypto, vfs, entropy, market) are registered once by the kernel spawn path
//! and never by a capsule at runtime. The runtime `sys_service_register`
//! syscall consults this to refuse a capsule that tries to register (squat) a
//! core name or port — e.g. after a core service crashes and before it
//! respawns, which would let a capsule impersonate keyring/crypto/vfs and
//! harvest secrets. The kernel spawn path does not go through the syscall, so
//! legitimate (re)registration is unaffected. The logic is dependency-free so
//! it can be proven on the host.
//!
//! The `net.*` sub-services (net.tcp/udp/dhcp.client/dns) are deliberately NOT
//! reserved here: net.core is a single capsule that publishes several endpoints
//! and must register them from user space at runtime through this syscall.
//! Squatting is instead barred by capability: `required_caps` forces
//! `IPC|Network` for every `net.*` name, so only a network-capable capsule can
//! claim one, and `register_endpoint` refuses a name/port already held.

const RESERVED_NAMES: [&str; 5] =
    ["keyring", "entropy_pool", "crypto_pool", "vfs_pool", "market.index"];

/// True if `name`/`port` belongs to a trusted core service and so must not be
/// registrable through the runtime service-register syscall.
pub(crate) fn is_reserved_service(name: &str, port: u32) -> bool {
    RESERVED_NAMES.contains(&name)
        // Core service + reply ports (keyring..market occupy 4098..=4107).
        || (4098..=4107).contains(&port)
}
