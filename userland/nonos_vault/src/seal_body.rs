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

//! Writing the record out, with the key already derived.
//!
//! Split from [`crate::seal`] so the key has one owner: the caller derives it,
//! passes it in, and wipes it whether this succeeds or fails. Nothing here
//! reaches for a machine root, which is also what lets the tests drive the
//! layout with a known key and no TPM.

use nonos_seal::{seal as aead_seal, NONCE_LEN};

use crate::blob::{header, CIPHERTEXT_AT, HEADER_LEN, NONCE_AT};
use crate::error::VaultError;

/// The header is written to `out` and passed to the AEAD as associated data,
/// so the bytes a reader will authenticate are the bytes that were written.
pub(crate) fn write(
    key: &[u8; 32],
    record: &[u8],
    plaintext: &[u8],
    nonce: &[u8; NONCE_LEN],
    out: &mut [u8],
) -> Result<usize, VaultError> {
    let head = header(record.len() as u8);
    out[..HEADER_LEN].copy_from_slice(&head);
    out[NONCE_AT..CIPHERTEXT_AT].copy_from_slice(nonce);
    match aead_seal(key, nonce, &head, plaintext, &mut out[CIPHERTEXT_AT..]) {
        Ok(n) => Ok(CIPHERTEXT_AT + n),
        Err(_) => Err(VaultError::BadLength),
    }
}
