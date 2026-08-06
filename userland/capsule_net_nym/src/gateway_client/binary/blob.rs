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

use super::kinds::{HEADER_BYTES, NONCE_BYTES};
use crate::crypto::gcm_siv::seal;
use crate::crypto::random::fill_random;
use crate::crypto::types::CryptoError;
use alloc::vec::Vec;

/// Wrap a payload as an encrypted gateway frame.
///
/// The kind and flag sit outside the sealed region, as in the reference: the
/// gateway reads them before deciding to decrypt. Nothing secret goes there,
/// and a peer on the path can change them freely.
pub fn make_encrypted_blob(kind: u8, plaintext: &[u8]) -> Result<Vec<u8>, CryptoError> {
    let key = crate::state::gateway_shared_key().ok_or(CryptoError::Kdf)?;
    let mut nonce = [0u8; NONCE_BYTES];
    fill_random(&mut nonce)?;
    let ciphertext = seal(&key, &nonce, &[], plaintext);
    let mut out = Vec::with_capacity(HEADER_BYTES + NONCE_BYTES + ciphertext.len());
    out.push(kind);
    out.push(1);
    out.extend_from_slice(&nonce);
    out.extend_from_slice(&ciphertext);
    Ok(out)
}
