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

//! `MkAttestEntries`: the set behind the root an attestation signs.
//!
//! `MkAttestDoc` returns a document whose authority is a TPM signature over
//! `registry_root`, a digest of every capsule running and what each is
//! permitted to do. A root on its own is an opaque value: a verifier can check
//! the TPM signed it and still not know what it means. This returns the
//! entries the root folds, so the verifier can recompute it and read the
//! capability mask of each program.
//!
//! That is what lets a machine make the claim worth making: not "trust that
//! the program handling this had no network", but a capability mask, inside a
//! set that folds to a root a TPM signed against the asker's own challenge.
//!
//! The bytes carry no authority of their own and are not required to. A caller
//! that changes one produces a set that no longer folds to the signed root,
//! which is precisely what the verifier is checking. So this hands out nothing
//! that has to be believed, and refusing to hand it out would only make the
//! signature it accompanies unusable.

use super::errnos::{ERRNO_FAULT, ERRNO_NOMEM};
use crate::security::attest_registry::registry_entries;

/// Writes the entries to `out_ptr` and returns how many bytes were written.
///
/// A buffer too small is refused without writing and without reporting the
/// size needed, matching `MkAttestDoc`: a caller that could probe for the
/// length would learn how much the machine is running without holding an
/// attestation of it. Ask with a large buffer and read the returned length.
pub fn sys_attest_entries(out_ptr: u64, out_len: u64) -> i64 {
    let entries = registry_entries();
    if entries.len() as u64 > out_len {
        return ERRNO_NOMEM;
    }
    if crate::usercopy::validate_user_write(out_ptr, entries.len()).is_err() {
        return ERRNO_FAULT;
    }
    match crate::usercopy::write_user_bytes(out_ptr, &entries) {
        Ok(()) => entries.len() as i64,
        Err(_) => ERRNO_FAULT,
    }
}
