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

//! The public half of the attestation key, out of the `TPM2_CreatePrimary`
//! response.
//!
//! The TPM answers the derivation with the object handle and then `outPublic`,
//! the `TPMT_PUBLIC` it derived. The point in it is the key a verifier needs
//! to check a quote's signature, and until now it was thrown away with the
//! rest of the response. Parsing is strict and accepts only the shape our own
//! template asks for: a part that answered with a different curve or scheme
//! is not the key this kernel meant, whatever else it might be.

use super::cursor::Cursor;
use super::attributes::OBJECT_ATTRIBUTES;
use crate::security::tpm::error::TpmError;

const TPM_ALG_ECC: u16 = 0x0023;
const TPM_ALG_SHA256: u16 = 0x000B;
const TPM_ALG_NULL: u16 = 0x0010;
const TPM_ALG_ECDSA: u16 = 0x0018;
const TPM_ECC_NIST_P256: u16 = 0x0003;

/// Header, object handle, parameter size: where `outPublic` begins.
const OUT_PUBLIC_AT: usize = 10 + 4 + 4;

/// The uncompressed P-256 point, x then y, 32 bytes each.
pub(super) fn parse_public(resp: &[u8]) -> Result<[u8; 64], TpmError> {
    let mut c = Cursor::at(resp, OUT_PUBLIC_AT);
    let size = c.u16()? as usize;
    if c.position() + size > resp.len() {
        return Err(TpmError::InvalidResponse);
    }
    let ok = c.u16()? == TPM_ALG_ECC && c.u16()? == TPM_ALG_SHA256;
    /*
     * The attributes are the property. A key without restricted, or not fixed
     * to this TPM, would sign anything handed to it, so the parse refuses
     * anything but the template's exact set.
     */
    let ok = ok && c.u32()? == OBJECT_ATTRIBUTES;
    let policy = c.u16()? as usize;
    c.skip(policy)?;
    let ok = ok && c.u16()? == TPM_ALG_NULL;
    let ok = ok && c.u16()? == TPM_ALG_ECDSA && c.u16()? == TPM_ALG_SHA256;
    let ok = ok && c.u16()? == TPM_ECC_NIST_P256 && c.u16()? == TPM_ALG_NULL;
    if !ok {
        return Err(TpmError::InvalidResponse);
    }
    let mut point = [0u8; 64];
    for half in point.chunks_exact_mut(32) {
        if c.u16()? != 32 {
            return Err(TpmError::InvalidResponse);
        }
        half.copy_from_slice(c.take(32)?);
    }
    Ok(point)
}
