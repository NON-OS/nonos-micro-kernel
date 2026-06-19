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
use crate::crypto::chacha20poly1305::aead_encrypt;
use crate::crypto::rng::fill_random_bytes;

pub fn seal(
    key: &[u8; 32],
    lba: u64,
    plain: &[u8],
) -> Result<[u8; SECTOR_BYTES], CryptoBlockError> {
    if plain.len() != PLAIN_BLOCK_BYTES {
        return Err(CryptoBlockError::InvalidLength);
    }
    let mut nonce = [0u8; NONCE_BYTES];
    fill_random_bytes(&mut nonce);
    let mut aad = [0u8; 24];
    aad[..16].copy_from_slice(AAD_PREFIX);
    aad[16..].copy_from_slice(&lba.to_le_bytes());
    let sealed = aead_encrypt(key, &nonce, &aad, plain)
        .map_err(|_| CryptoBlockError::AuthenticationFailed)?;
    let mut sector = [0u8; SECTOR_BYTES];
    sector[..NONCE_BYTES].copy_from_slice(&nonce);
    sector[NONCE_BYTES..].copy_from_slice(&sealed);
    Ok(sector)
}
