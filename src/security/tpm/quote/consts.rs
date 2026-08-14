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

/// Command tag for a command carrying an authorization area. Quote always
/// does, because signing with the attestation key requires authorization even
/// when that authorization is the empty password.
pub(super) const TPM_ST_SESSIONS: u16 = 0x8002;

/// `TPM_ST_ATTEST_QUOTE`, the type field inside the signed structure. A
/// verifier checks it: a signature over some other attest type is not a quote,
/// and accepting one would let a different command's output be replayed here.
pub(super) const TPM_ST_ATTEST_QUOTE: u16 = 0x8018;

pub(super) const TPM_CC_QUOTE: u32 = 0x0000_0158;

/// The password session handle. Used with an empty HMAC, which is the correct
/// authorization for an attestation key created with no auth value.
pub(super) const TPM_RS_PW: u32 = 0x4000_0009;

pub(super) const TPM_ALG_NULL: u16 = 0x0010;
pub(super) const TPM_ALG_SHA256: u16 = 0x000B;

/// `TPM_RC_SUCCESS`.
pub(super) const TPM_RC_SUCCESS: u32 = 0;

/// Header of every TPM2 command and response: tag, size, code.
pub(super) const TPM_HEADER_LEN: usize = 10;

/// Bytes of PCR selection bitmap for a bank of 24 PCRs.
pub(super) const PCR_SELECT_BYTES: usize = 3;

/// Magic prefix of a `TPMS_ATTEST`, `TPM_GENERATED_VALUE`. Its presence is
/// what proves the structure was produced inside the TPM rather than supplied
/// by the caller, so a verifier that skips it accepts forged attestations.
pub const TPM_GENERATED_VALUE: u32 = 0xFF54_4347;
