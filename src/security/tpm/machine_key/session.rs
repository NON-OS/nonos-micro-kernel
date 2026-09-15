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

//! `TPM2_StartAuthSession` for a policy session.
//!
//! Unbound and unsalted, so the session key is empty and the HMAC on later
//! commands may be too. The session's worth is not secrecy, it is that the TPM
//! and only the TPM writes its policy digest.

use alloc::vec::Vec;

use super::consts::{
    NONCE_LEN, TPM_ALG_NULL, TPM_ALG_SHA256, TPM_CC_START_AUTH_SESSION, TPM_RH_NULL, TPM_SE_POLICY,
    TPM_ST_NO_SESSIONS,
};
use super::error::KeyError;
use super::wire::{checked, frame, u32_at};

pub(super) fn build_start(nonce: &[u8; NONCE_LEN]) -> Vec<u8> {
    let mut body = Vec::with_capacity(40);
    body.extend_from_slice(&TPM_RH_NULL.to_be_bytes());
    body.extend_from_slice(&TPM_RH_NULL.to_be_bytes());
    /*
     * nonceCaller: the TPM answers with a nonce of its own of the same size,
     * and the caller's decides the size of both.
     */
    body.extend_from_slice(&(NONCE_LEN as u16).to_be_bytes());
    body.extend_from_slice(nonce);
    // encryptedSalt: none, with a null tpmKey there is nothing to encrypt to.
    body.extend_from_slice(&0u16.to_be_bytes());
    body.push(TPM_SE_POLICY);
    // symmetric: TPMT_SYM_DEF with a null algorithm and so no key bits or mode.
    body.extend_from_slice(&TPM_ALG_NULL.to_be_bytes());
    body.extend_from_slice(&TPM_ALG_SHA256.to_be_bytes());
    frame(TPM_ST_NO_SESSIONS, TPM_CC_START_AUTH_SESSION, &body)
}

/// The session handle. The TPM's nonce follows it and is not needed: with no
/// HMAC to compute there is nothing to fold it into.
pub(super) fn parse_start(resp: &[u8]) -> Result<u32, KeyError> {
    let r = checked(resp)?;
    u32_at(r, 10)
}
