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

//! `TPM2_PolicyPCR` and `TPM2_PolicyGetDigest`.
//!
//! The first folds the selected PCRs, as they are right now, into the session.
//! The second reads the result back, and that value goes into the template of
//! the key: the TPM computed it from the real PCRs, so nothing here needs to
//! know a PCR value or hash one.

use alloc::vec::Vec;

use super::consts::{
    DIGEST_LEN, PCR_SELECT_BYTES, TPM_ALG_SHA256, TPM_CC_POLICY_GET_DIGEST, TPM_CC_POLICY_PCR,
    TPM_ST_NO_SESSIONS,
};
use super::error::KeyError;
use super::pcrs::bitmap;
use super::wire::{checked, digest_at, frame};

pub(super) fn build_policy_pcr(session: u32, pcrs: &[u8]) -> Vec<u8> {
    let mut body = Vec::with_capacity(20);
    body.extend_from_slice(&session.to_be_bytes());
    /*
     * pcrDigest: empty, which tells the TPM to use the PCRs as they are rather
     * than check them against a value we assert.
     */
    body.extend_from_slice(&0u16.to_be_bytes());
    // pcrs: one TPMS_PCR_SELECTION over the SHA-256 bank.
    body.extend_from_slice(&1u32.to_be_bytes());
    body.extend_from_slice(&TPM_ALG_SHA256.to_be_bytes());
    body.push(PCR_SELECT_BYTES as u8);
    body.extend_from_slice(&bitmap(pcrs));
    frame(TPM_ST_NO_SESSIONS, TPM_CC_POLICY_PCR, &body)
}

pub(super) fn build_get_digest(session: u32) -> Vec<u8> {
    frame(TPM_ST_NO_SESSIONS, TPM_CC_POLICY_GET_DIGEST, &session.to_be_bytes())
}

/// A bare success is all `PolicyPCR` answers with.
pub(super) fn parse_policy_pcr(resp: &[u8]) -> Result<(), KeyError> {
    checked(resp).map(|_| ())
}

pub(super) fn parse_get_digest(resp: &[u8]) -> Result<[u8; DIGEST_LEN], KeyError> {
    let r = checked(resp)?;
    digest_at(r, 10)
}
