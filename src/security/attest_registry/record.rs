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

use super::complete::mark_incomplete;
use super::entry::AttestedCapsule;
use super::table::TABLE;
use crate::security::dev_roots::Authority;

/// Record a capsule that just passed the spawn gate.
///
/// `measurement` must come from the verifier, not be recomputed by the caller:
/// the registry's whole value is that it states what was proved.
///
/// A full table marks the registry incomplete rather than failing the spawn.
/// The capsule is already running by this point, so refusing here would not
/// unrun it; what it would do is let the machine keep attesting while missing
/// an entry. Instead attestation stops until reboot.
pub fn record_attested(pid: u32, measurement: [u8; 32], caps: u64, authority: Authority) {
    if !TABLE.lock().insert(AttestedCapsule { pid, measurement, caps, authority }) {
        mark_incomplete();
        crate::sys::serial::println(
            b"[ATTEST] registry full; attestation disabled until reboot",
        );
    }
}

/// Drop a capsule that has exited. Absent entries are not an error: teardown
/// runs for capsules that never reached the attestation gate.
pub fn forget_attested(pid: u32) {
    let _ = TABLE.lock().remove(pid);
}
