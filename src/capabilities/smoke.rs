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

//! Single source for the smoketest-only `Capability::Debug` grant.
//!
//! `MkDebug` is gated by `Capability::Debug`. Production builds do
//! not grant that bit to any process; only the boot smoketests need
//! capsule-emitted serial markers, and only those builds OR this
//! function's result into a process's `caps_bits` at mint time.
//!
//! Two consumers exist:
//!   - `process::core::table::inherit::init_caps_bits` (init's
//!     ambient set; proof_io reaches MkDebug through that).
//!   - `kernel_core::process_spawn::capsule_spawn::runner::install_caps`
//!     (every spawned capsule).
//!
//! Both call sites use this helper so the cfg list lives in one
//! place and a new smoketest profile only has to touch this file.

use super::Capability;

#[cfg(any(
    feature = "nonos-user-entry-proof",
    feature = "nonos-ramfs-smoketest",
    feature = "nonos-keyring-smoketest",
    feature = "nonos-entropy-smoketest",
    feature = "nonos-crypto-hash-smoketest",
    feature = "nonos-vfs-smoketest",
    feature = "nonos-driver-virtio-rng-smoketest",
    feature = "nonos-driver-virtio-blk-smoketest",
    feature = "nonos-driver-virtio-net-smoketest",
    feature = "nonos-driver-ps2-input-smoketest",
    feature = "nonos-driver-xhci-smoketest",
    feature = "nonos-market-smoketest",
    feature = "nonos-wallpaper-smoketest",
))]
pub fn debug_grant() -> u64 {
    Capability::Debug.bit()
}

#[cfg(not(any(
    feature = "nonos-user-entry-proof",
    feature = "nonos-ramfs-smoketest",
    feature = "nonos-keyring-smoketest",
    feature = "nonos-entropy-smoketest",
    feature = "nonos-crypto-hash-smoketest",
    feature = "nonos-vfs-smoketest",
    feature = "nonos-driver-virtio-rng-smoketest",
    feature = "nonos-driver-virtio-blk-smoketest",
    feature = "nonos-driver-virtio-net-smoketest",
    feature = "nonos-driver-ps2-input-smoketest",
    feature = "nonos-driver-xhci-smoketest",
    feature = "nonos-market-smoketest",
    feature = "nonos-wallpaper-smoketest",
)))]
pub fn debug_grant() -> u64 {
    let _ = Capability::Debug;
    0
}
