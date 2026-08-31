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

use super::consts::{TPM_GENERATED_VALUE, TPM_ST_ATTEST_QUOTE};
use super::error::QuoteError;

/// Check a `TPMS_ATTEST` before anyone relies on it, and confirm it answers
/// the challenge that was issued.
///
/// Three properties, and all three are load-bearing. The magic proves the TPM
/// built the structure rather than the caller supplying it. The type proves it
/// is a quote and not some other attestation replayed into this position. The
/// nonce proves it is this quote and not a recording of an earlier honest one.
/// A verifier that checks the signature alone accepts all three attacks.
pub fn check_attest(attest: &[u8], expected_nonce: &[u8]) -> Result<(), QuoteError> {
    if attest.len() < 6 {
        return Err(QuoteError::Truncated);
    }
    let magic = u32::from_be_bytes([attest[0], attest[1], attest[2], attest[3]]);
    if magic != TPM_GENERATED_VALUE {
        return Err(QuoteError::NotTpmGenerated);
    }
    if u16::from_be_bytes([attest[4], attest[5]]) != TPM_ST_ATTEST_QUOTE {
        return Err(QuoteError::NotAQuote);
    }

    // qualifiedSigner is a TPM2B; skip it to reach extraData, which carries
    // the nonce back.
    let mut pos = 6usize;
    pos = skip_tpm2b(attest, pos)?;

    if pos + 2 > attest.len() {
        return Err(QuoteError::Truncated);
    }
    let n = u16::from_be_bytes([attest[pos], attest[pos + 1]]) as usize;
    let start = pos + 2;
    let end = start.checked_add(n).ok_or(QuoteError::Truncated)?;
    if end > attest.len() {
        return Err(QuoteError::Truncated);
    }
    if &attest[start..end] != expected_nonce {
        return Err(QuoteError::NonceMismatch);
    }
    Ok(())
}

fn skip_tpm2b(buf: &[u8], pos: usize) -> Result<usize, QuoteError> {
    if pos + 2 > buf.len() {
        return Err(QuoteError::Truncated);
    }
    let n = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
    let end = pos.checked_add(2).and_then(|p| p.checked_add(n)).ok_or(QuoteError::Truncated)?;
    if end > buf.len() {
        return Err(QuoteError::Truncated);
    }
    Ok(end)
}
