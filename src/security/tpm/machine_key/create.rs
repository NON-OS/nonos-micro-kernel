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

//! `TPM2_CreatePrimary` for the keyed-hash object the key comes out of.
//!
//! Deterministic in the same way the attestation key is: seed plus template,
//! and the template carries the policy digest, so the object is a function of
//! the boot state before any policy is checked.

use alloc::vec::Vec;

use super::consts::{
    DIGEST_LEN, OBJECT_ATTRIBUTES, TPM_ALG_HMAC, TPM_ALG_KEYEDHASH, TPM_ALG_SHA256,
    TPM_CC_CREATE_PRIMARY, TPM_RH_OWNER, TPM_ST_SESSIONS,
};
use super::error::KeyError;
use super::wire::{checked, frame, u32_at};

/// The password session handle, for the owner hierarchy's own empty auth.
const TPM_RS_PW: u32 = 0x4000_0009;

pub(super) fn build_create(policy: &[u8; DIGEST_LEN]) -> Vec<u8> {
    let template = template(policy);
    let mut body = Vec::with_capacity(32 + template.len());
    body.extend_from_slice(&TPM_RH_OWNER.to_be_bytes());
    let mut auth = [0u8; 9];
    auth[..4].copy_from_slice(&TPM_RS_PW.to_be_bytes());
    body.extend_from_slice(&(auth.len() as u32).to_be_bytes());
    body.extend_from_slice(&auth);
    // inSensitive: no auth value and no caller data, the TPM supplies the key.
    body.extend_from_slice(&4u16.to_be_bytes());
    body.extend_from_slice(&0u16.to_be_bytes());
    body.extend_from_slice(&0u16.to_be_bytes());
    body.extend_from_slice(&(template.len() as u16).to_be_bytes());
    body.extend_from_slice(&template);
    // outsideInfo empty, creationPCR none
    body.extend_from_slice(&0u16.to_be_bytes());
    body.extend_from_slice(&0u32.to_be_bytes());
    frame(TPM_ST_SESSIONS, TPM_CC_CREATE_PRIMARY, &body)
}

/// TPMT_PUBLIC: keyed-hash, HMAC-SHA256, authorised by the policy alone.
fn template(policy: &[u8; DIGEST_LEN]) -> Vec<u8> {
    let mut p = Vec::with_capacity(48);
    p.extend_from_slice(&TPM_ALG_KEYEDHASH.to_be_bytes());
    p.extend_from_slice(&TPM_ALG_SHA256.to_be_bytes());
    p.extend_from_slice(&OBJECT_ATTRIBUTES.to_be_bytes());
    p.extend_from_slice(&(DIGEST_LEN as u16).to_be_bytes());
    p.extend_from_slice(policy);
    // TPMS_KEYEDHASH_PARMS: scheme HMAC over SHA-256.
    p.extend_from_slice(&TPM_ALG_HMAC.to_be_bytes());
    p.extend_from_slice(&TPM_ALG_SHA256.to_be_bytes());
    // unique: empty, filled by the derivation.
    p.extend_from_slice(&0u16.to_be_bytes());
    p
}

/// The object handle leads the response, ahead of the parameter area.
pub(super) fn parse_create(resp: &[u8]) -> Result<u32, KeyError> {
    let r = checked(resp)?;
    u32_at(r, 10)
}
