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

use alloc::vec::Vec;
use crate::crypto::ed25519_verify::verify as ed25519_verify;

use super::error::CredentialError;
use super::types::StoredCredential;
use crate::crypto::blake3;
use crate::state;

const MIN_LEN: usize = 8 + 32 + 64 + 32;
const MAX_LEN: usize = 1024;

pub fn parse(body: &[u8], now_ms: u64) -> Result<StoredCredential, CredentialError> {
    if body.len() < MIN_LEN || body.len() > MAX_LEN {
        return Err(CredentialError::BadLength);
    }
    let expiry = expiry(body);
    if expiry == 0 || expiry <= now_ms {
        return Err(CredentialError::BadExpiry);
    }
    let issuer = &body[8..40];
    match state::trusted_authority(issuer) {
        Some(true) => {}
        Some(false) => return Err(CredentialError::UntrustedAuthority),
        None => return Err(CredentialError::NoAuthority),
    }
    let sig = &body[40..104];
    let payload = &body[104..];
    let msg = credential_message(&body[0..8], payload);
    if ed25519_verify(issuer, sig, &msg) != 0 {
        return Err(CredentialError::BadSignature);
    }
    let mut material = [0u8; 32];
    blake3(body, &mut material).map_err(|_| CredentialError::Crypto)?;
    let mut key = [0u8; 32];
    key.copy_from_slice(issuer);
    Ok(StoredCredential { expiry_ms: expiry, issuer: key, material })
}

fn credential_message(expiry: &[u8], payload: &[u8]) -> Vec<u8> {
    let mut msg = Vec::with_capacity(expiry.len() + payload.len());
    msg.extend_from_slice(expiry);
    msg.extend_from_slice(payload);
    msg
}

fn expiry(body: &[u8]) -> u64 {
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&body[0..8]);
    u64::from_le_bytes(bytes)
}
