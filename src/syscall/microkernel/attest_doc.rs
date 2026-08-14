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

//! `MkAttestDoc`: a signed statement of what this machine is running.
//!
//! Unlike `MkAttestStatus`, which reports what the bootloader recorded, this
//! produces a document a third party can check: the TPM signs the boot
//! measurements together with a digest of every capsule currently running,
//! under a challenge the asker chose.
//!
//! The challenge comes from the caller and is never generated here. A nonce
//! the machine picks for itself proves nothing to anybody else.

use super::errnos::{ERRNO_FAULT, ERRNO_NOMEM, ERRNO_PERM};
use crate::security::attest_doc::attest;

/// Callers must supply exactly this many bytes of challenge. A short challenge
/// is refused rather than padded, because padding would let a caller narrow
/// the space an attacker has to precompute.
const CHALLENGE_LEN: usize = 32;

pub fn sys_attest_doc(challenge_ptr: u64, out_ptr: u64, out_len: u64) -> i64 {
    let mut challenge = [0u8; CHALLENGE_LEN];
    if crate::usercopy::copy_from_user(challenge_ptr, &mut challenge).is_err() {
        return ERRNO_FAULT;
    }

    let doc = match attest(&challenge) {
        Ok(doc) => doc,
        // The reason is on the console, not in the errno: a caller that could
        // distinguish "no TPM" from "registry incomplete" learns about the
        // machine's state without being entitled to an attestation of it.
        Err(e) => {
            crate::sys::serial::print(b"[ATTEST] refused: ");
            crate::sys::serial::println(e.as_str().as_bytes());
            return ERRNO_PERM;
        }
    };

    let encoded = doc.encode();
    if encoded.len() as u64 > out_len {
        // The length is not returned on overflow. A caller sizing a buffer
        // should ask with a large one and read the returned length, rather
        // than probing to learn how much the machine is running.
        return ERRNO_NOMEM;
    }
    if crate::usercopy::write_user_bytes(out_ptr, &encoded).is_err() {
        return ERRNO_FAULT;
    }
    encoded.len() as i64
}
