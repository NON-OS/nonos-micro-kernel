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

use core::sync::atomic::{AtomicU32, Ordering};

use super::create::build_create_primary;
use crate::security::tpm::crb::transact;
use crate::security::tpm::error::TpmError;

/// Handle of the loaded attestation key, or zero before bring-up. Transient:
/// the TPM forgets it at reset, which costs nothing because the same
/// derivation reproduces it on the next boot.
static AK_HANDLE: AtomicU32 = AtomicU32::new(0);

const RESPONSE_MAX: usize = 2048;

/// Derive and load the attestation key. Idempotent, because a second
/// derivation would occupy another of the very few transient object slots a
/// TPM has for no benefit.
pub fn load_ak() -> Result<u32, TpmError> {
    let existing = AK_HANDLE.load(Ordering::Acquire);
    if existing != 0 {
        return Ok(existing);
    }
    let cmd = build_create_primary();
    let mut buf = [0u8; RESPONSE_MAX];
    // SAFETY: eK@nonos.systems - deriving a primary key creates a transient
    // object. It writes no NV state and cannot displace an existing key.
    let len = unsafe { transact(&cmd, &mut buf) }?;
    let handle = parse_handle(&buf[..len])?;
    match AK_HANDLE.compare_exchange(0, handle, Ordering::AcqRel, Ordering::Acquire) {
        Ok(_) => Ok(handle),
        Err(winner) => Ok(winner),
    }
}

/// The loaded key, or `None` before bring-up ran.
pub fn ak_handle() -> Option<u32> {
    match AK_HANDLE.load(Ordering::Acquire) {
        0 => None,
        h => Some(h),
    }
}

/// `TPM2_CreatePrimary` answers with the object handle first, ahead of the
/// parameter area, so the response code must be checked before those four
/// bytes mean anything.
fn parse_handle(resp: &[u8]) -> Result<u32, TpmError> {
    if resp.len() < 14 {
        return Err(TpmError::InvalidResponse);
    }
    let code = u32::from_be_bytes([resp[6], resp[7], resp[8], resp[9]]);
    if code != 0 {
        return Err(TpmError::InvalidResponse);
    }
    let handle = u32::from_be_bytes([resp[10], resp[11], resp[12], resp[13]]);
    if handle == 0 {
        return Err(TpmError::InvalidResponse);
    }
    Ok(handle)
}
