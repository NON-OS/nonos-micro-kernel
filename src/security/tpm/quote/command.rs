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

use super::consts::{
    PCR_SELECT_BYTES, TPM_ALG_NULL, TPM_ALG_SHA256, TPM_CC_QUOTE, TPM_RS_PW, TPM_ST_SESSIONS,
};
use super::pcr::pcr_bitmap;

/// Build a `TPM2_Quote` over `pcrs` in the SHA-256 bank, bound to `nonce`.
///
/// The nonce travels as `qualifyingData` and the TPM copies it into the signed
/// structure. That copy is the entire freshness guarantee: a verifier that
/// does not compare it against the challenge it issued will accept a quote
/// captured from an earlier honest boot.
///
/// The signing scheme is `TPM_ALG_NULL`, meaning the key's own scheme is used.
/// A restricted attestation key carries one, and naming a different scheme
/// here would be refused by the TPM rather than silently honoured.
pub fn build_quote(ak_handle: u32, nonce: &[u8], pcrs: &[u8]) -> Vec<u8> {
    let mut body = Vec::with_capacity(64 + nonce.len());

    body.extend_from_slice(&ak_handle.to_be_bytes());

    // Authorization area: a password session with an empty HMAC. Its length
    // prefix covers only what follows it.
    let auth: [u8; 9] = {
        let mut a = [0u8; 9];
        a[..4].copy_from_slice(&TPM_RS_PW.to_be_bytes());
        // nonce size 0, attributes 0, hmac size 0 are already zero.
        a
    };
    body.extend_from_slice(&(auth.len() as u32).to_be_bytes());
    body.extend_from_slice(&auth);

    // qualifyingData: TPM2B_DATA.
    body.extend_from_slice(&(nonce.len() as u16).to_be_bytes());
    body.extend_from_slice(nonce);

    // inScheme: TPMT_SIG_SCHEME with a null algorithm and therefore no
    // scheme-specific detail following it.
    body.extend_from_slice(&TPM_ALG_NULL.to_be_bytes());

    // PCRselect: one TPMS_PCR_SELECTION over the SHA-256 bank.
    body.extend_from_slice(&1u32.to_be_bytes());
    body.extend_from_slice(&TPM_ALG_SHA256.to_be_bytes());
    body.push(PCR_SELECT_BYTES as u8);
    body.extend_from_slice(&pcr_bitmap(pcrs));

    let mut cmd = Vec::with_capacity(10 + body.len());
    cmd.extend_from_slice(&TPM_ST_SESSIONS.to_be_bytes());
    cmd.extend_from_slice(&((10 + body.len()) as u32).to_be_bytes());
    cmd.extend_from_slice(&TPM_CC_QUOTE.to_be_bytes());
    cmd.extend_from_slice(&body);
    cmd
}
