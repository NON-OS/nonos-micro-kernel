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

//! `TPM2_HMAC`, authorised by the policy session.

use alloc::vec::Vec;

use super::consts::{DIGEST_LEN, TPM_ALG_SHA256, TPM_CC_HMAC, TPM_ST_SESSIONS};
use super::error::KeyError;
use super::wire::{checked, digest_at, frame};

pub(super) fn build_hmac(key: u32, session: u32, label: &[u8]) -> Vec<u8> {
    let mut body = Vec::with_capacity(24 + label.len());
    body.extend_from_slice(&key.to_be_bytes());
    /*
     * Authorisation area: the policy session, no nonce, no attributes, and an
     * empty HMAC, which the TPM accepts because the session key is empty and
     * the object has no auth value to fold in.
     */
    let mut auth = [0u8; 9];
    auth[..4].copy_from_slice(&session.to_be_bytes());
    body.extend_from_slice(&(auth.len() as u32).to_be_bytes());
    body.extend_from_slice(&auth);
    // buffer: TPM2B_MAX_BUFFER holding the label.
    body.extend_from_slice(&(label.len() as u16).to_be_bytes());
    body.extend_from_slice(label);
    body.extend_from_slice(&TPM_ALG_SHA256.to_be_bytes());
    frame(TPM_ST_SESSIONS, TPM_CC_HMAC, &body)
}

/// With a session on the command the response carries a parameter size ahead
/// of the digest, so the TPM2B sits at 14 rather than 10.
pub(super) fn parse_hmac(resp: &[u8]) -> Result<[u8; DIGEST_LEN], KeyError> {
    let r = checked(resp)?;
    digest_at(r, 14)
}
