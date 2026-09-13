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

use crate::syscall::{call_raw, N_MK_ATTEST_DOC, N_MK_ATTEST_ENTRIES, N_MK_ATTEST_STATUS};

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AttestStatus {
    pub zk_verified: u8,
    pub kernel_sig_ok: u8,
    pub secure_boot: u8,
    pub zk_attestation_ok: u8,
    pub kernel_blake3: [u8; 32],
    pub program_hash: [u8; 32],
}

pub extern "C" fn mk_attest_status(out: *mut AttestStatus) -> i64 {
    call_raw(N_MK_ATTEST_STATUS, [out as u64, 0, 0, 0, 0, 0])
}

/// Errno the kernel returns when it will not attest: no TPM, or a capsule
/// registry it could not complete. It is deliberately the same value for both,
/// so a caller cannot probe the machine's state through the failure code.
pub const ATTEST_DOC_REFUSED: i64 = -1;

/// Ask the machine for a signed statement of what it is running.
///
/// The challenge is folded into the qualifying data the TPM signs, which is what
/// stops a document being replayed: a verifier supplies its own and checks it
/// comes back. Returns the encoded document's length, or a negative errno. On
/// overflow the length is not returned, so a caller sizes generously and reads
/// the length rather than probing upward to learn how much is running.
pub fn mk_attest_doc(challenge: &[u8; 32], out: &mut [u8]) -> i64 {
    call_raw(
        N_MK_ATTEST_DOC,
        [challenge.as_ptr() as u64, out.as_mut_ptr() as u64, out.len() as u64, 0, 0, 0],
    )
}

/// Bytes per entry: pid, measurement, capability mask, authority.
pub const ATTEST_ENTRY_LEN: usize = 45;

/// Read the capsule entries the attestation registry root folds.
///
/// A signed root is a digest of these, so a verifier recomputes it from them
/// and, if it matches, knows which programs were running and what each was
/// permitted to do. Without them the signature is over a number nobody can
/// interpret. Carries no authority: altering an entry produces a set that no
/// longer folds to the signed root, which is exactly what the verifier checks.
/// Returns the bytes written, or a negative errno. A buffer too small is
/// refused without saying how short, so size for the whole registry.
pub fn mk_attest_entries(out: &mut [u8]) -> i64 {
    call_raw(N_MK_ATTEST_ENTRIES, [out.as_mut_ptr() as u64, out.len() as u64, 0, 0, 0, 0])
}
