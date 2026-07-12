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

//! Capsule serial-debug capability grant, gated behind an explicit build
//! feature so a hardened release can drop it entirely.
//!
//! The `Debug` capability lets a capsule write to the serial console via
//! `mk_debug`. That is useful for bring-up diagnostics but is a deliberate
//! information-leak / attack surface in a shipped system: any capsule holding
//! it can emit arbitrary bytes to the host serial line, and it is treated as a
//! forbidden ambient capability elsewhere in the process table. Granting it to
//! long-lived service capsules (net.core, net.sockets, ...) therefore widens
//! the trusted surface with no production benefit.
//!
//! Rather than hard-wire `Capability::Debug.bit()` into each capsule's grant,
//! sites use [`serial_debug_cap`]. With the `capsule-serial-debug` feature the
//! grant is unchanged; without it every capsule is denied serial output, so a
//! release build strips the surface by dropping a single feature flag instead
//! of editing every spawn site.

#[cfg(feature = "capsule-serial-debug")]
use super::types::Capability;

/// Serial-debug capability bits to fold into a capsule's requested caps.
/// Returns `Debug` only when the `capsule-serial-debug` feature is enabled,
/// otherwise `0`.
#[cfg(feature = "capsule-serial-debug")]
pub(crate) const fn serial_debug_cap() -> u64 {
    Capability::Debug.bit()
}

/// See the enabled variant above. A hardened build compiles this one and no
/// capsule can obtain the `Debug` capability through a spawn grant.
#[cfg(not(feature = "capsule-serial-debug"))]
pub(crate) const fn serial_debug_cap() -> u64 {
    0
}
