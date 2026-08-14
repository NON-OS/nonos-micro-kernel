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

use super::consts::{TPM_HEADER_LEN, TPM_RC_SUCCESS};
use super::error::QuoteError;

/// The two halves a verifier needs: the structure the TPM signed, and the
/// signature over it.
///
/// `attest` is kept as raw bytes rather than parsed into fields. A verifier
/// must hash exactly what was signed, and re-serialising a parsed structure is
/// how a byte of padding or an unhandled field silently changes the digest.
pub struct QuoteResult<'a> {
    pub attest: &'a [u8],
    pub signature: &'a [u8],
}

/// Split a `TPM2_Quote` response into the signed structure and its signature.
///
/// The response code is checked first: a failed command still returns a
/// well-formed header, and treating its body as a quote would parse whatever
/// follows as attestation data.
pub fn parse_quote(resp: &[u8]) -> Result<QuoteResult<'_>, QuoteError> {
    if resp.len() < TPM_HEADER_LEN {
        return Err(QuoteError::Truncated);
    }
    let size = u32::from_be_bytes([resp[2], resp[3], resp[4], resp[5]]) as usize;
    let code = u32::from_be_bytes([resp[6], resp[7], resp[8], resp[9]]);
    if code != TPM_RC_SUCCESS {
        return Err(QuoteError::Tpm(code));
    }
    if size > resp.len() || size < TPM_HEADER_LEN {
        return Err(QuoteError::Truncated);
    }

    // TPM2B_ATTEST: a 16-bit length then that many bytes.
    let body = &resp[TPM_HEADER_LEN..size];
    if body.len() < 2 {
        return Err(QuoteError::Truncated);
    }
    let attest_len = u16::from_be_bytes([body[0], body[1]]) as usize;
    let attest_end = 2 + attest_len;
    if attest_end > body.len() {
        return Err(QuoteError::Truncated);
    }
    let attest = &body[2..attest_end];

    // Everything after is the TPMT_SIGNATURE. It is handed back whole because
    // its shape depends on the key's algorithm, and the verifier owns that.
    let signature = &body[attest_end..];
    if signature.is_empty() {
        return Err(QuoteError::Truncated);
    }
    Ok(QuoteResult { attest, signature })
}
