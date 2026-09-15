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

//! TPM 2.0 constants the derivation speaks in. Part 2 of the library
//! specification, by name.

pub(super) const TPM_ST_NO_SESSIONS: u16 = 0x8001;
pub(super) const TPM_ST_SESSIONS: u16 = 0x8002;

pub(super) const TPM_CC_CREATE_PRIMARY: u32 = 0x0000_0131;
pub(super) const TPM_CC_HMAC: u32 = 0x0000_0155;
pub(super) const TPM_CC_FLUSH_CONTEXT: u32 = 0x0000_0165;
pub(super) const TPM_CC_START_AUTH_SESSION: u32 = 0x0000_0176;
pub(super) const TPM_CC_POLICY_PCR: u32 = 0x0000_017F;
pub(super) const TPM_CC_POLICY_GET_DIGEST: u32 = 0x0000_0189;

/// The owner hierarchy. Its seed changes on `TPM2_Clear`, so a key under it
/// dies with the owner's data, which is the right lifetime for a volume key.
/// The endorsement hierarchy, which the attestation key lives under, survives
/// a clear and must: identity is not data.
pub(super) const TPM_RH_OWNER: u32 = 0x4000_0001;
pub(super) const TPM_RH_NULL: u32 = 0x4000_0007;

pub(super) const TPM_SE_POLICY: u8 = 0x01;

pub(super) const TPM_ALG_KEYEDHASH: u16 = 0x0008;
pub(super) const TPM_ALG_HMAC: u16 = 0x0005;
pub(super) const TPM_ALG_SHA256: u16 = 0x000B;
pub(super) const TPM_ALG_NULL: u16 = 0x0010;

/// `fixedTPM | fixedParent | sensitiveDataOrigin | sign`. No `userWithAuth`,
/// so the object's only authorisation is its policy: the empty password that
/// authorises the attestation key does not authorise this one.
pub(super) const OBJECT_ATTRIBUTES: u32 = 0x0004_0032;

pub(super) const HEADER_LEN: usize = 10;
pub(super) const DIGEST_LEN: usize = 32;
pub(super) const NONCE_LEN: usize = 16;
pub(super) const PCR_SELECT_BYTES: usize = 3;

/// TPM2B_MAX_BUFFER allows 1024. A label is a name, not a payload.
pub const LABEL_MAX: usize = 64;
