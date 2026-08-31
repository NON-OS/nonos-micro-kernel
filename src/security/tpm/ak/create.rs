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

use super::template::ak_template;

const TPM_ST_SESSIONS: u16 = 0x8002;
const TPM_CC_CREATE_PRIMARY: u32 = 0x0000_0131;
const TPM_RS_PW: u32 = 0x4000_0009;

/// The endorsement hierarchy. Its seed is set at manufacture and survives an
/// owner clear, so a key derived under it identifies the physical part rather
/// than the current owner's configuration of it.
const TPM_RH_ENDORSEMENT: u32 = 0x4000_000B;

/// Build `TPM2_CreatePrimary` for the attestation key.
///
/// Nothing about the request is random, which is the point: the TPM derives
/// the key from its seed and this exact byte sequence, so the same part
/// answers with the same key on every boot.
pub(super) fn build_create_primary() -> Vec<u8> {
    let template = ak_template();
    let mut body = Vec::with_capacity(48 + template.len());

    body.extend_from_slice(&TPM_RH_ENDORSEMENT.to_be_bytes());

    // Authorization: password session with an empty HMAC.
    let auth = [0u8; 9];
    let mut auth_area = auth;
    auth_area[..4].copy_from_slice(&TPM_RS_PW.to_be_bytes());
    body.extend_from_slice(&(auth_area.len() as u32).to_be_bytes());
    body.extend_from_slice(&auth_area);

    // inSensitive: TPM2B_SENSITIVE_CREATE wrapping an empty auth value and no
    // caller-supplied secret, so the key material comes from the TPM.
    body.extend_from_slice(&4u16.to_be_bytes());
    body.extend_from_slice(&0u16.to_be_bytes());
    body.extend_from_slice(&0u16.to_be_bytes());

    body.extend_from_slice(&template);

    // outsideInfo empty, and no PCRs bound into creation: the key must be
    // derivable in any boot state, or it could not attest to a state it does
    // not already match.
    body.extend_from_slice(&0u16.to_be_bytes());
    body.extend_from_slice(&0u32.to_be_bytes());

    let mut cmd = Vec::with_capacity(10 + body.len());
    cmd.extend_from_slice(&TPM_ST_SESSIONS.to_be_bytes());
    cmd.extend_from_slice(&((10 + body.len()) as u32).to_be_bytes());
    cmd.extend_from_slice(&TPM_CC_CREATE_PRIMARY.to_be_bytes());
    cmd.extend_from_slice(&body);
    cmd
}
