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

use super::error::AttestError;
use super::proved::Proved;
use crate::security::dev_roots::{authority_for, enrolled_roots, Authority};

/// Gate a spawn on the capsule's attestation.
///
/// The vendor root is tried first and always. Only if that fails are enrolled
/// developer roots attempted, so a capsule that verifies under the shipped
/// policy is never attributed to a local key, and a local key can never
/// shadow the vendor's answer.
///
/// Returns what was proved and who proved it. The caller records both: a
/// measurement without its authority is a claim that something was verified
/// without saying against what, which is the kind of half-truth attestation
/// exists to eliminate.
#[must_use = "a capsule must not be spawned unless its attestation verifies"]
pub fn verify_capsule_attestation(
    trailer: &[u8],
    elf: &[u8],
    granted_caps: u64,
) -> Result<Proved, AttestError> {
    let vendor = super::policy_root::root().ok_or(AttestError::RootUnavailable)?;
    if let Ok(measurement) = super::against_root::verify(trailer, elf, granted_caps, &vendor) {
        return Ok(Proved { measurement, authority: Authority::Vendor });
    }

    let (roots, n) = enrolled_roots();
    for root in roots.iter().take(n) {
        if let Ok(measurement) = super::against_root::verify(trailer, elf, granted_caps, root) {
            // The slot is looked up rather than inferred from the loop index,
            // so the reported authority is the table's answer and cannot drift
            // from it if the table is reordered.
            let authority = authority_for(root).ok_or(AttestError::Rejected)?;
            return Ok(Proved { measurement, authority });
        }
    }
    Err(AttestError::Rejected)
}
