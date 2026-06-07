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

// The capsule attestation trailer, appended after the dual signatures:
//
//   magic  "NZKCAPS1" (8 bytes)
//   proof  u32 LE length + Groth16 / BLS12-381 proof bytes
//   pubins u32 LE length + N * 32-byte big-endian field elements
//   commit 32 bytes

use super::error::AttestError;
use super::layout::{FE, PI_COUNT};

const TRAILER_MAGIC: &[u8; 8] = b"NZKCAPS1";

pub(super) struct Trailer<'a> {
    pub proof: &'a [u8],
    pub public_inputs: &'a [u8],
    pub commitment: [u8; 32],
}

fn read_u32_le(buf: &[u8], off: usize) -> Option<usize> {
    let b = buf.get(off..off + 4)?;
    Some(u32::from_le_bytes([b[0], b[1], b[2], b[3]]) as usize)
}

pub(super) fn parse(blob: &[u8]) -> Result<Trailer<'_>, AttestError> {
    if blob.len() < TRAILER_MAGIC.len() || &blob[0..TRAILER_MAGIC.len()] != TRAILER_MAGIC {
        return Err(AttestError::Missing);
    }
    let mut off = TRAILER_MAGIC.len();

    let proof_len = read_u32_le(blob, off).ok_or(AttestError::Malformed)?;
    off += 4;
    let proof = blob.get(off..off + proof_len).ok_or(AttestError::Malformed)?;
    off += proof_len;

    let pubin_len = read_u32_le(blob, off).ok_or(AttestError::Malformed)?;
    off += 4;
    if pubin_len % FE != 0 || pubin_len / FE != PI_COUNT {
        return Err(AttestError::Malformed);
    }
    let public_inputs = blob.get(off..off + pubin_len).ok_or(AttestError::Malformed)?;
    off += pubin_len;

    let commit_slice = blob.get(off..off + 32).ok_or(AttestError::Malformed)?;
    let mut commitment = [0u8; 32];
    commitment.copy_from_slice(commit_slice);

    Ok(Trailer { proof, public_inputs, commitment })
}
