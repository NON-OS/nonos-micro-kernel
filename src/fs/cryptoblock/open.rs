// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::constants::{AAD_PREFIX, NONCE_BYTES, PLAIN_BLOCK_BYTES, SECTOR_BYTES};
use super::CryptoBlockError;
use crate::crypto::chacha20poly1305::aead_decrypt;

pub fn open(
    key: &[u8; 32],
    lba: u64,
    sector: &[u8; SECTOR_BYTES],
) -> Result<[u8; PLAIN_BLOCK_BYTES], CryptoBlockError> {
    let mut nonce = [0u8; NONCE_BYTES];
    nonce.copy_from_slice(&sector[..NONCE_BYTES]);
    let mut aad = [0u8; 24];
    aad[..16].copy_from_slice(AAD_PREFIX);
    aad[16..].copy_from_slice(&lba.to_le_bytes());
    let plain = aead_decrypt(key, &nonce, &aad, &sector[NONCE_BYTES..])
        .map_err(|_| CryptoBlockError::AuthenticationFailed)?;
    if plain.len() != PLAIN_BLOCK_BYTES {
        return Err(CryptoBlockError::InvalidLength);
    }
    let mut out = [0u8; PLAIN_BLOCK_BYTES];
    out.copy_from_slice(&plain);
    Ok(out)
}
