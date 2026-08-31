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

use super::authority::Authority;
use super::consent;
use super::error::EnrolError;
use super::table::TABLE;
use crate::capabilities::Capability;
use crate::security::attest_registry::registry_complete;

/// Ask to enrol a signing root. Does not enrol anything.
///
/// Enrolment is two steps on purpose. This one checks that the request is
/// permissible and shows the user a code; only `confirm_dev_root` with that
/// code actually widens what the machine will run. A capsule can therefore
/// request, but cannot approve.
pub fn request_dev_root(caller_caps: u64, root: [u8; 32]) -> Result<(), EnrolError> {
    if caller_caps & Capability::EnrolDevRoot.bit() == 0 {
        return Err(EnrolError::Denied);
    }
    // A machine that has already lost track of what it is running must not
    // gain new authorities: it would be adding code its own attestation
    // cannot describe.
    if !registry_complete() {
        return Err(EnrolError::RegistryIncomplete);
    }
    // An all-zero root is what an uninitialised buffer looks like.
    if root == [0u8; 32] {
        return Err(EnrolError::EmptyRoot);
    }
    if TABLE.lock().is_full() {
        return Err(EnrolError::NoSlots);
    }
    consent::arm_challenge(root).ok_or(EnrolError::EntropyUnavailable)?;
    Ok(())
}

/// Enrol this machine's own build root.
///
/// The caller does not supply it. A capsule that chose the root could enrol
/// somebody else's tree and then run anything its holder signed.
pub fn request_local_build_root(caller_caps: u64) -> Result<(), EnrolError> {
    let root = crate::security::local_build::root().ok_or(EnrolError::EmptyRoot)?;
    request_dev_root(caller_caps, root)
}

/// Complete the pending enrolment.
///
/// The capability is checked again rather than assumed from the request. The
/// two calls may arrive from different capsules, and a caller that could
/// confirm without holding the right would only need to wait for somebody
/// else's request to be in flight.
pub fn confirm_dev_root(caller_caps: u64, answer: u32) -> Result<Authority, EnrolError> {
    if caller_caps & Capability::EnrolDevRoot.bit() == 0 {
        return Err(EnrolError::Denied);
    }
    let root = consent::redeem(answer).ok_or(EnrolError::NotConfirmed)?;
    let slot = TABLE.lock().insert(root).ok_or(EnrolError::NoSlots)?;
    crate::sys::serial::print(b"[DEV-ROOT] enrolled slot=");
    crate::sys::serial::print_hex(slot as u64);
    crate::sys::serial::println(b"; locally built capsules may now run");
    Ok(Authority::Developer(slot))
}

/// How many developer roots this session holds. Always zero immediately after
/// a boot: nothing enrolled survives the power going off.
pub fn dev_root_count() -> usize {
    TABLE.lock().roots.iter().filter(|s| s.used).count()
}
