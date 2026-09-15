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

//! Taking a record apart before anything trusts it.

use nonos_seal::NONCE_LEN;

use crate::blob::{CIPHERTEXT_AT, HEADER_LEN, MAGIC, NONCE_AT, OVERHEAD, VERSION};
use crate::error::VaultError;

/// A record split into the three parts opening it needs.
pub struct Parsed<'a> {
    /// The record's own header bytes, which are the associated data. Handed
    /// back rather than rebuilt so the caller authenticates what is there.
    pub header: &'a [u8],
    pub nonce: [u8; NONCE_LEN],
    pub ciphertext: &'a [u8],
}

/// Length, magic and version are checked before anything else, so a file that
/// is not a record costs no TPM round trip and no key derivation.
///
/// The declared record-name length is compared against the caller's here
/// rather than only inside the AEAD. Both refuse, but failing on the header
/// means a record for another name is rejected without deriving that name's
/// key, and the reason is a distinguishable one.
pub fn parse(bytes: &[u8], record_len: u8) -> Result<Parsed<'_>, VaultError> {
    if bytes.len() <= OVERHEAD || bytes[..8] != MAGIC {
        return Err(VaultError::NotAVault);
    }
    if u16::from_le_bytes([bytes[8], bytes[9]]) != VERSION || bytes[10] != record_len {
        return Err(VaultError::NotAVault);
    }
    let mut nonce = [0u8; NONCE_LEN];
    nonce.copy_from_slice(&bytes[NONCE_AT..CIPHERTEXT_AT]);
    Ok(Parsed { header: &bytes[..HEADER_LEN], nonce, ciphertext: &bytes[CIPHERTEXT_AT..] })
}
