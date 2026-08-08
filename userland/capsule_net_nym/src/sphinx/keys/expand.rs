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

use super::super::constants::{
    EXPANDED_SHARED_SECRET_HKDF_INFO, EXPANDED_SHARED_SECRET_HKDF_SALT,
    EXPANDED_SHARED_SECRET_LENGTH,
};
use super::types::ExpandedSharedSecret;
use crate::crypto::kdf::hkdf_sha256;
use crate::crypto::types::CryptoError;

/// Cut one hop's Diffie-Hellman result into the five keys the hop needs.
pub fn expand_shared_secret(secret: &[u8; 32]) -> Result<ExpandedSharedSecret, CryptoError> {
    let mut out = [0u8; EXPANDED_SHARED_SECRET_LENGTH];
    hkdf_sha256(
        EXPANDED_SHARED_SECRET_HKDF_SALT,
        secret,
        EXPANDED_SHARED_SECRET_HKDF_INFO,
        &mut out,
    )?;
    Ok(ExpandedSharedSecret(out))
}
