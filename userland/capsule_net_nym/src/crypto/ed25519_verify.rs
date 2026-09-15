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

//! Signature verification, on its own because its return value inverts.
//!
//! This is what decides whether the directory's view of the mixnet is the
//! directory's, so it is the function in this capsule that must not be got
//! wrong. It kept the syscall's errno shape when it moved out of the kernel:
//! **zero is a good signature** and non-zero is a rejection, which is the
//! opposite of what a function called `verify` reads like. Every call site
//! tests `!= 0` as a rejection. Returning a bool here, or returning 1 for
//! success, silently accepts every forgery the mixnet is handed.

use nonos_ed25519::{verify as ed_verify, Signature};

use super::ed25519::{PUBLIC_LEN, SIGNATURE_LEN};

/// The verify contract, errno shaped: zero is success.
const GOOD: i64 = 0;
const BAD: i64 = -1;

/// A short key or signature is a rejection rather than a panic: both arrive
/// from the network, and this capsule runs with `panic = "abort"`.
pub fn verify(public: &[u8], signature: &[u8], message: &[u8]) -> i64 {
    if public.len() < PUBLIC_LEN || signature.len() < SIGNATURE_LEN {
        return BAD;
    }
    let mut pk = [0u8; PUBLIC_LEN];
    pk.copy_from_slice(&public[..PUBLIC_LEN]);
    let mut sig = [0u8; SIGNATURE_LEN];
    sig.copy_from_slice(&signature[..SIGNATURE_LEN]);

    if ed_verify(&pk, message, &Signature::from_bytes(&sig)) {
        GOOD
    } else {
        BAD
    }
}
