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

pub(super) fn spawn_ramfs() {
    use crate::fs::ramfs_capsule as c;
    super::boot::capsule("RAMFS", "ramfs", c::spawn_ramfs_capsule, c::shared_state);
}

pub(super) fn spawn_after_ramfs() {
    spawn_keyring();
    spawn_entropy();
    spawn_crypto();
    spawn_policy();
}

pub(super) fn spawn_vfs() {
    use crate::fs::vfs_capsule as c;

    super::boot::capsule("VFS", "vfs", c::spawn_vfs_capsule, c::shared_state);
}

#[cfg(feature = "nonos-capsule-market")]
pub(super) fn spawn_market() {
    use crate::security::market_capsule as c;

    super::boot::capsule("MARKET", "market", c::spawn_market_capsule, c::shared_state);
}

#[cfg(not(feature = "nonos-capsule-market"))]
pub(super) fn spawn_market() {}

fn spawn_keyring() {
    use crate::security::keyring_capsule as c;

    super::boot::capsule("KEYRING", "keyring", c::spawn_keyring_capsule, c::shared_state);
}

fn spawn_entropy() {
    use crate::security::entropy_capsule as c;

    super::boot::capsule("ENTROPY", "entropy", c::spawn_entropy_capsule, c::shared_state);
}

fn spawn_crypto() {
    use crate::security::crypto_capsule as c;

    super::boot::capsule("CRYPTO", "crypto", c::spawn_crypto_capsule, c::shared_state);
}

#[cfg(feature = "nonos-capsule-policy")]
fn spawn_policy() {
    use crate::userspace::capsule_policy as c;

    super::boot::capsule("POLICY", "policy", c::spawn_policy_capsule, c::shared_state);
}

#[cfg(not(feature = "nonos-capsule-policy"))]
fn spawn_policy() {}
