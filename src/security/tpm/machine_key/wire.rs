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

//! Framing shared by every command here, and the two reads every response
//! needs.

use alloc::vec::Vec;

use super::consts::{DIGEST_LEN, HEADER_LEN};
use super::error::KeyError;
use crate::security::tpm::error::TpmError;

/// Header, then body. The size field covers the header too.
pub(super) fn frame(tag: u16, code: u32, body: &[u8]) -> Vec<u8> {
    let mut cmd = Vec::with_capacity(HEADER_LEN + body.len());
    cmd.extend_from_slice(&tag.to_be_bytes());
    cmd.extend_from_slice(&((HEADER_LEN + body.len()) as u32).to_be_bytes());
    cmd.extend_from_slice(&code.to_be_bytes());
    cmd.extend_from_slice(body);
    cmd
}

/// The response code, as an error when it is not success. Checked before any
/// other byte of a response is read: on failure the TPM sends a bare header
/// and the fields a parser would expect are simply not there.
pub(super) fn checked(resp: &[u8]) -> Result<&[u8], KeyError> {
    if resp.len() < HEADER_LEN {
        return Err(TpmError::InvalidResponse.into());
    }
    let rc = u32::from_be_bytes([resp[6], resp[7], resp[8], resp[9]]);
    if rc != 0 {
        return Err(KeyError::Refused(rc));
    }
    Ok(resp)
}

pub(super) fn u32_at(b: &[u8], at: usize) -> Result<u32, KeyError> {
    let s = b.get(at..at + 4).ok_or(TpmError::InvalidResponse)?;
    Ok(u32::from_be_bytes([s[0], s[1], s[2], s[3]]))
}

/// A `TPM2B_DIGEST` that must be exactly SHA-256 sized. Shorter is not a
/// weaker key, it is a parse against the wrong offset.
pub(super) fn digest_at(b: &[u8], at: usize) -> Result<[u8; DIGEST_LEN], KeyError> {
    let size = b.get(at..at + 2).ok_or(TpmError::InvalidResponse)?;
    if u16::from_be_bytes([size[0], size[1]]) as usize != DIGEST_LEN {
        return Err(TpmError::InvalidResponse.into());
    }
    let body = b.get(at + 2..at + 2 + DIGEST_LEN).ok_or(TpmError::InvalidResponse)?;
    let mut out = [0u8; DIGEST_LEN];
    out.copy_from_slice(body);
    Ok(out)
}
